grammar lolspeakGrammar;

lolcode
    : HAI comments? head? body KTHXBYE EOF
    ;

head
    : MAEK HEAD comments? title OIC
    ;

title
    : GIMMEH TITLE TEXT MKAY
    ;

comments
    : comment+
    ;

comment
    : OBTW TEXT TLDR
    ;

body
    : (comments? bodyElement)*
    ;

bodyElement
    : paragraph
    | bold
    | italics
    | list
    | audio
    | video
    | newline
    | variableDefine
    | variableUse
    | TEXT
    ;

paragraph
    : MAEK PARAGRAF comments? variableDefine? innerParagraph OIC
    ;

innerParagraph
    : (comments? innerElement)*
    ;

innerElement
    : bold
    | italics
    | list
    | audio
    | video
    | newline
    | variableUse
    | TEXT
    ;

list
    : MAEK LIST comments? variableDefine? listItems OIC
    ;

listItems
    : (comments? listItem)*
    ;

listItem
    : GIMMEH ITEM listItemContent MKAY
    ;

listItemContent
    : (bold | italics | variableUse | TEXT)*
    ;

bold
    : GIMMEH BOLD TEXT MKAY
    ;

italics
    : GIMMEH ITALICS TEXT MKAY
    ;

audio
    : GIMMEH SOUNDZ TEXT MKAY
    ;

video
    : GIMMEH VIDZ TEXT MKAY
    ;

newline
    : GIMMEH NEWLINE
    ;

variableDefine
    : IHASH HAZ IDENT ITHASH IZ TEXT MKAY
    ;

variableUse
    : LEMME SEE IDENT MKAY
    ;






HAI      : '#' ('H'|'h') ('A'|'a') ('I'|'i') ;
KTHXBYE  : '#' ('K'|'k') ('T'|'t') ('H'|'h') ('X'|'x') ('B'|'b') ('Y'|'y') ('E'|'e') ;
MAEK     : '#' ('M'|'m') ('A'|'a') ('E'|'e') ('K'|'k') ;
GIMMEH   : '#' ('G'|'g') ('I'|'i') ('M'|'m') ('M'|'m') ('E'|'e') ('H'|'h') ;
OIC      : '#' ('O'|'o') ('I'|'i') ('C'|'c') ;
OBTW     : '#' ('O'|'o') ('B'|'b') ('T'|'t') ('W'|'w') ;
TLDR     : '#' ('T'|'t') ('L'|'l') ('D'|'d') ('R'|'r') ;
MKAY     : '#' ('M'|'m') ('K'|'k') ('A'|'a') ('Y'|'y') ;
IHASH    : '#' ('I'|'i') ;
ITHASH   : '#' ('I'|'i') ('T'|'t') ;
LEMME    : '#' ('L'|'l') ('E'|'e') ('M'|'m') ('M'|'m') ('E'|'e') ;


HEAD     : ('H'|'h') ('E'|'e') ('A'|'a') ('D'|'d') ;
TITLE    : ('T'|'t') ('I'|'i') ('T'|'t') ('L'|'l') ('E'|'e') ;
PARAGRAF : ('P'|'p') ('A'|'a') ('R'|'r') ('A'|'a') ('G'|'g') ('R'|'r') ('A'|'a') ('F'|'f') ;
LIST     : ('L'|'l') ('I'|'i') ('S'|'s') ('T'|'t') ;
ITEM     : ('I'|'i') ('T'|'t') ('E'|'e') ('M'|'m') ;
BOLD     : ('B'|'b') ('O'|'o') ('L'|'l') ('D'|'d') ;
ITALICS  : ('I'|'i') ('T'|'t') ('A'|'a') ('L'|'l') ('I'|'i') ('C'|'c') ('S'|'s') ;
SOUNDZ   : ('S'|'s') ('O'|'o') ('U'|'u') ('N'|'n') ('D'|'d') ('Z'|'z') ;
VIDZ     : ('V'|'v') ('I'|'i') ('D'|'d') ('Z'|'z') ;
NEWLINE  : ('N'|'n') ('E'|'e') ('W'|'w') ('L'|'l') ('I'|'i') ('N'|'n') ('E'|'e') ;
SEE      : ('S'|'s') ('E'|'e') ('E'|'e') ;
HAZ      : ('H'|'h') ('A'|'a') ('Z'|'z') ;
IZ       : ('I'|'i') ('Z'|'z') ;


TEXT
    : (~('#'|'\r'|'\n'))+
    ;


IDENT
    : ('A'..'Z'|'a'..'z'|'_') ('A'..'Z'|'a'..'z'|'0'..'9'|'_')*
    ;


WS
    : (' ' | '\t' | '\r' | '\n')+ { $channel = HIDDEN; }
    ;