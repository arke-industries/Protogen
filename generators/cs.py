# C# generator

boilerplate = """
using System;
using System.ComponentModel.DataAnnotations;

using ArkeIndustries.RequestServer;
using ArkeIndustries.RequestServer.DataAnnotations;

#pragma warning disable 0649

namespace ArkeIndustries.Starfall.Api {
    public sealed class ServerId {
        % for server in ast.servers:
            public static ushort ${server.name} { get; } = %{loop.index};
        % endfor
    }

    public sealed class NotificationType {
        % for notif in ast.notifications:
            public static ushort ${notif.name} { get; } = ${loop.index};
        % endfor
    }

    % for cat in categories:
        namespace ${cat.name} {
            % for meth in cat.methods:
                [MessageServer(ServerId = ServerId.${meth.server_id})]
                public partial class ${meth.name} : ${meth.base_class} {
                    public override ushort Category { get; } = ${loop.parent.index};
                    public override ushort Method { get; } = ${loop.index};

                    ${meth.method_fields}
                }
            % endfor
        }
    % endfor
}
"""

def process_types(ast):
    "Replace references to types with their correct type, and replace with the C# typename"
    pass

def propagate_server_attrs(ast):
    "Bubble `server = Global` etc references down to unmarked methods"
    pass

def fix_methods(ast):
    "Simplify the complex property structure of methods and format the fields of the classes"
    pass

def generate(ast):
    process_types(ast)
    propagate_server_attrs(ast)
    fix_methods(ast)
    pass
