generated_code_styling_html = ""
generated_line_number_html = ""

line_numbers = 0

JAVA_DATA_TYPES = ["short", "byte", "int", "long", "char", "String", "float", "double"]
JAVA_MODULE_KEYWORDS = ["import"]
JAVA_NAMESPACE_KEYWORDS = ["package"]
JAVA_CONSTRUCT_KEYWORDS = ["abstract", "continue", "for", "new", "switch",
                "assert", "default", "synchronized", "do", "if", "private",
                "this", "break", "implements", "protected", "throw", "throws",
                "case", "instanceof", "return", "transient", "catch", "extends",
                "try", "final", "interface", "static", "void", "class", "finally",
                "volatile", "strictfp", "const", "native", "super", "while",
                "public"]

JAVA_PRIMITVE_VALUES = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "true", "false"]


def read_file(pathToFile):

    global line_numbers, generated_code_styling_html, generated_line_number_html

    file_object = open(pathToFile, "r")

    for line in file_object:
        line_numbers += 1
        generated_line_number_html += "{}<br>\n".format(str(line_numbers))
        for word in line.split(" "):
            word_html = ""

            if word in JAVA_DATA_TYPES:
                word_html = '<span class="data-type-keyword">{}</span>'.format(word)
            elif word in JAVA_MODULE_KEYWORDS:
                word_html = '<span class="module">{}</span><br>'.format(word)
            elif word in JAVA_NAMESPACE_KEYWORDS:
                word_html = '<span class="namespace">{}</span><br><br>'.format(word)
            elif word in JAVA_CONSTRUCT_KEYWORDS:
                word_html = '<span class="construct-keyword">{}</span>'.format(word)
            elif word in JAVA_PRIMITVE_VALUES:
                word_html = '<span class="data-type-value-keyword">{}</span>'.format(word)
            elif word == "\n":
                word_html = "<br>\n"
            else:
                word_html = word+" "

            generated_code_styling_html += word_html

read_file("./test.java");
print(generated_line_number_html)
print(generated_code_styling_html)

