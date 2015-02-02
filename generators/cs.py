# C# generator

boilerplate = """\
using System;
using System.ComponentModel.DataAnnotations;

using ArkeIndustries.RequestServer;
using ArkeIndustries.RequestServer.DataAnnotations;

#pragma warning disable 0649

namespace ArkeIndustries.Starfall.Api {
    public sealed class ServerId {
        % for server in ast["servers"]:
        public const int ${server["name"]} = ${2**loop.index};
        % endfor
    }

    public sealed class NotificationType {
        % for notif in ast["notifications"]:
        public static ushort ${notif["name"]} { get; } = ${loop.index};
        % endfor
    }

    public sealed class ResponseCode : ArkeIndustries.RequestServer.ResponseCode {
        % for resp in ast["response_codes"]:
        public static ushort ${resp["name"]} { get; } = ${loop.index + 100};
        % endfor
    }

    % for cat in ast["categories"]:
    namespace ${cat["name"]} {
        % for meth in cat["methods"]:
        [MessageDefinition(ServerId = ${meth["server_ids"]}, AuthenticationRequired = ${"auth" in meth["meta"] and meth["meta"]["auth"]})]
        % for attr in meth["attrs"]:
        ${attr}
        % endfor
        public partial class ${meth["name"]} : ${meth["base_class"]} {
            public override ushort Category { get; } = ${loop.parent.index};
            public override ushort Method { get; } = ${loop.index};

            % if meth["is_list"]:
            % for inp in meth["inputs"]:
            [MessageInput(Index = ${meth["name"]}.InputStartIndex + ${loop.index})]
            % for attr in inp["attrs"]:
            ${attr}
            % endfor
            public ${inp["type"]} ${inp["name"]} { get; set; }
            % endfor
            public class ${meth["list_class_name"]} {
                % for out in meth["outputs"]:
                [MessageOutput(Index = ${loop.index})]
                % for attr in out["attrs"]:
                ${attr}
                % endfor
                public ${out["type"]} ${out["name"]} { get; set; }

                % endfor
            }
            % else:
            % for inp in meth["inputs"]:
            [MessageInput(Index = ${loop.index})]
            % for attr in inp["attrs"]:
            ${attr}
            % endfor
            public ${inp["type"]} ${inp["name"]} { get; set; }

            % endfor
            % for out in meth["outputs"]:
            [MessageOutput(Index = ${loop.index})]
            % for attr in out["attrs"]:
            ${attr}
            % endfor
            public ${out["type"]} ${out["name"]} { get; set; }

            % endfor
            % endif
        }
        % endfor
    }
    % endfor
}
"""

builtins = {
    "u64": "ulong",
    "i64": "long",
    "u32": "uint",
    "i32": "int",
    "u16": "ushort",
    "i16": "short",
    "u8": "byte",
    "i8": "sbyte"
}

def trans(ast, typename):
    if typename in builtins:
        return builtins[typename]
    else:
        for t in ast["types"]:
            if typename == t["name"]:
                return trans(ast, t["type"])
        # well, we tried. assume it's meaningful for now
        return typename

def process_types(ast):
    "Replace references to types with their correct type, and replace with the C# typename"
    # hilariously bad exponential algorithm
    while True:
        touched_any = False
        for t1 in ast["types"]:
            t1["val"] = trans(ast, t1["val"])
            for t2 in ast["types"]:
                if t2["val"] == t1["name"]:
                    t2["val"] = trans(ast, t1["val"])
                    touched_any = True
                    break

        if not touched_any:
            break

def propagate_server_attrs(ast):
    for cat in ast["categories"]:
        for meth in cat["methods"]:
            meth["meta"] = {}
        for prop in cat["properties"]:
            for meth in cat["methods"]:
                if prop["name"] not in meth["meta"]:
                    if prop["name"] == "server":
                        if "server" not in meth["meta"]:
                            meth["meta"]["server"] = []
                        meth["meta"]["server"].append(prop["val"])
                    else:
                        meth["meta"][prop["name"]] = prop["val"]

def sattr(a):
    if a["plain"] is not None:
        return a["plain"]
    else:
        if len(a["args"]) != 0:
            return "%s(%s)" % (a["name"], ", ".join(sattr(a) for a in a["args"] if len(a) != 0))
        else:
            return "%s = %s" % (a["name"], a["val"])

def normalize_methods(ast):
    "Simplify the complex property structure of methods and format the fields of the classes"
    for cat in ast["categories"]:
        for meth in cat["methods"]:
            if any(map(lambda a: a["name"] == "List", meth["attributes"])):
                lattr = [m for m in meth["attributes"] if m["name"] == "List"][0]
                # simplify list attributes
                meth["list_class_name"] = [n for n in lattr["args"] if "name" in n and n["name"] == "Class"][0]["val"].replace('"', '')
                meth["base_class"] = "ListQueryMessageHandler<Objects.Database.Context, %s.%s>" % (meth["name"], meth["list_class_name"])
            else:
                meth["base_class"] = "MessageHandler<Objects.Database.Context>"

            meth["server_ids"] = " | ".join("ServerId.%s" % s for s in meth["meta"]["server"])
            meth["attrs"] = "\n".join("[%s]" % sattr(a) for a in meth["attributes"])

            inps = [m for m in meth["properties"] if m["name"] == "in" ]
            outs = [m for m in meth["properties"] if m["name"] == "out"]

            meth["inputs"] = []
            meth["outputs"] = []

            if inps:
                for inp in inps[0]["type"]["obj"]:
                    meth["inputs"].append({
                        "name": inp["name"],
                        "type": trans(ast, inp["type"]),
                        "attrs": ["[%s]" % sattr(a) for a in inp["attributes"]]
                    })

            if outs:
                for out in outs[0]["type"]["obj"]:
                    meth["outputs"].append({
                        "name": out["name"],
                        "type": trans(ast, out["type"]),
                        "attrs": ["[%s]" % sattr(a) for a in out["attributes"]]
                    })

def generate(ast):
    from mako.template import Template

    process_types(ast)
    propagate_server_attrs(ast)
    normalize_methods(ast)

    return Template(boilerplate).render(ast=ast)
