parser.py: grammar.ebnf
	PYTHONPATH=../.. python -m grako -m Protogen -o $@ $< 2>&1

clean:
	-@rm -f parser.py
