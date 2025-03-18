
const DATA_ADJ: &str = r#"
    00001740 00 a 01 able 0 005 = 05200169 n 0000 = 05616246 n 0000 + 05616246 n 0101 + 05200169 n 0101 ! 00002098 a 0101 | (usually followed by `to') having the necessary means or skill or know-how or authority to do something; "able to swim"; "she was able to program her computer"; "we were at last able to buy a car"; "able to get a grant for the project"  
    00002098 00 a 01 unable 0 002 = 05200169 n 0000 ! 00001740 a 0101 | (usually followed by `to') not having the necessary means or skill or know-how; "unable to get to town without a car"; "unable to obtain funds"  
    00002312 00 a 02 abaxial 0 dorsal 4 002 ;c 06037666 n 0000 ! 00002527 a 0101 | facing away from the axis of an organ or organism; "the abaxial surface of a leaf is the underside or side facing away from the stem"  
    00002527 00 a 02 adaxial 0 ventral 4 002 ;c 06037666 n 0000 ! 00002312 a 0101 | nearest to or facing toward the axis of an organ or organism; "the upper side of a leaf is known as the adaxial surface"  
    00002730 00 a 01 acroscopic 0 002 ;c 06066555 n 0000 ! 00002843 a 0101 | facing or on the side toward the apex  
    00002843 00 a 01 basiscopic 0 002 ;c 06066555 n 0000 ! 00002730 a 0101 | facing or on the side toward the base  
    00002956 00 a 02 abducent 0 abducting 0 002 ;c 06080522 n 0000 ! 00003131 a 0101 | especially of muscles; drawing away from the midline of the body or from an adjacent part  
"#;

// data.adv lines

const DATA_ADV: &str = r#"
00001740 02 r 01 a_cappella 0 000 | without musical accompaniment; "they performed a cappella"  
00001837 02 r 03 AD 0 A.D. 0 anno_Domini 0 000 | in the Christian era; used before dates after the supposed year Christ was born; "in AD 200"  
00001981 02 r 03 CE 0 C.E. 0 Common_Era 0 000 | of the period coinciding with the Christian era; preferred by some writers who are not Christians; "in 200 CE"  
00002142 02 r 03 BC 0 B.C. 0 before_Christ 0 000 | before the Christian era; used following dates before the supposed year Christ was born; "in 200 BC"  
00002296 02 r 02 BCE 0 B.C.E. 0 000 | of the period before the Common Era; preferred by some writers who are not Christians; "in 200 BCE"  
00002436 02 r 03 horseback 0 ahorse 0 ahorseback 0 000 | on the back of a horse; "he rode horseback to town"; "managed to escape ahorse"; "policeman patrolled the streets ahorseback"  
00002621 02 r 05 barely 0 hardly 0 just 4 scarcely 0 scarce 0 000 | only a very short time before; "they could barely hear the speaker"; "we hardly knew them"; "just missed being hit"; "had scarcely rung the bell when the door flew open"; "would have scarce arrived before she would have found some excuse to leave"- W.B.Yeats  
00002950 02 r 01 just 6 000 | exactly at this moment or the moment described; "we've just finished painting the walls, so don't touch them";  
00003093 02 r 02 hardly 2 scarcely 2 001 \ 00016756 a 0201 | almost not; "he hardly ever goes fishing"; "he was hardly more than sixteen years old"; "they scarcely ever used the emergency generator"  
00003294 02 r 01 anisotropically 0 001 \ 01361107 a 0101 | in an anisotropic manner  
00003380 02 r 01 annoyingly 0 001 \ 00089550 a 0101 | in an annoying manner or to an annoying degree  
00003483 02 r 03 basically 0 fundamentally 0 essentially 1 002 \ 00900616 a 0301 \ 01856419 a 0201 | in essence; at bottom or by one's (or its) very nature; "He is basically dishonest"; "the argument was essentially a technical one"; "for all his bluster he is in essence a shy person"  
00003771 02 r 01 blessedly 0 001 \ 00670741 a 0101 | in a blessed manner  
00003846 02 r 01 boiling 0 001 ;u 07075172 n 0000 | extremely; "boiling mad"  
00003925 02 r 01 enviably 0 001 \ 00733541 a 0101 | in an enviable manner; "she was enviably fluent in French"  
"#;

const DATA_NOUN: &str = r#"
00001740 03 n 01 entity 0 003 ~ 00001930 n 0000 ~ 00002137 n 0000 ~ 04424418 n 0000 | that which is perceived or known or inferred to have its own distinct existence (living or nonliving)  
00001930 03 n 01 physical_entity 0 007 @ 00001740 n 0000 ~ 00002452 n 0000 ~ 00002684 n 0000 ~ 00007347 n 0000 ~ 00020827 n 0000 ~ 00029677 n 0000 ~ 14580597 n 0000 | an entity that has physical existence  
00002137 03 n 02 abstraction 0 abstract_entity 0 010 @ 00001740 n 0000 + 00692329 v 0101 ~ 00023100 n 0000 ~ 00024264 n 0000 ~ 00031264 n 0000 ~ 00031921 n 0000 ~ 00033020 n 0000 ~ 00033615 n 0000 ~ 05810143 n 0000 ~ 07999699 n 0000 | a general concept formed by extracting common features from specific examples  
00002452 03 n 01 thing 0 009 @ 00001930 n 0000 ~ 04347225 n 0000 ~ 09225146 n 0000 ~ 09312645 n 0000 ~ 09367203 n 0000 ~ 09385911 n 0000 ~ 09407867 n 0000 ~ 09465459 n 0000 ~ 09468959 n 0000 | a separate and self-contained entity  
00002684 03 n 02 object 0 physical_object 0 039 @ 00001930 n 0000 + 00532607 v 0105 ~ 00003553 n 0000 ~ 00027167 n 0000 ~ 03009633 n 0000 ~ 03149951 n 0000 ~ 03233423 n 0000 ~ 03338648 n 0000 ~ 03532080 n 0000 ~ 03595179 n 0000 ~ 03610270 n 0000 ~ 03714721 n 0000 ~ 03892891 n 0000 ~ 04012260 n 0000 ~ 04248010 n 0000 ~ 04345288 n 0000 ~ 04486445 n 0000 ~ 07851054 n 0000 ~ 09238143 n 0000 ~ 09251689 n 0000 ~ 09267490 n 0000 ~ 09279458 n 0000 ~ 09281777 n 0000 ~ 09283193 n 0000 ~ 09287968 n 0000 ~ 09295338 n 0000 ~ 09300905 n 0000 ~ 09302031 n 0000 ~ 09308398 n 0000 ~ 09334396 n 0000 ~ 09335240 n 0000 ~ 09358550 n 0000 ~ 09368224 n 0000 ~ 09407346 n 0000 ~ 09409203 n 0000 ~ 09432990 n 0000 ~ 09468237 n 0000 ~ 09474162 n 0000 ~ 09477037 n 0000 | a tangible and visible entity; an entity that can cast a shadow; "it was full of rackets, balls and other objects"  
00003553 03 n 02 whole 0 unit 0 015 @ 00002684 n 0000 + 01462005 v 0204 + 00367685 v 0201 + 01385458 v 0201 + 00368109 v 0201 + 00784215 a 0103 ~ 00003993 n 0000 ~ 00004258 n 0000 ~ 00019128 n 0000 ~ 00021939 n 0000 ~ 02749953 n 0000 ~ 03588414 n 0000 %p 03892891 n 0000 %p 04164989 n 0000 ~ 04353803 n 0000 | an assemblage of parts that is regarded as a single entity; "how big is that part compared to the whole?"; "the team is a unit"  
00003993 03 n 01 congener 0 001 @ 00003553 n 0000 | a whole (a thing or person) of the same kind or category as another; "lard was also used, though its congener, butter, was more frequently employed"; "the American shopkeeper differs from his European congener"  
00004258 03 n 02 living_thing 0 animate_thing 0 007 @ 00003553 n 0000 -c 01646941 a 0000 ~ 00004475 n 0000 ~ 00006269 n 0000 ~ 00006400 n 0000 ~ 00006484 n 0000 -c 05056234 n 0000 | a living (or once living) entity  
00004475 03 n 02 organism 0 being 0 065 @ 00004258 n 0000 + 02614181 v 0201 + 02986509 a 0102 + 01679459 a 0101 + 01093142 a 0101 -c 00270856 a 0000 -c 00327031 a 0000 -c 01665816 a 0000 ~ 00005787 n 0000 ~ 00005930 n 0000 ~ 00006024 n 0000 ~ 00006150 n 0000 %p 00006484 n 0000 ~ 00007846 n 0000 ~ 00015388 n 0000 ~ 00017222 n 0000 ~ 00019046 n 0000 ~ 01313888 n 0000 ~ 01314026 n 0000 ~ 01314145 n 0000 ~ 01315062 n 0000 ~ 01319932 n 0000 ~ 01320093 n 0000 ~ 01320314 n 0000 ~ 01320479 n 0000 ~ 01320692 n 0000 ~ 01324019 n 0000 ~ 01326291 n 0000 ~ 01326897 n 0000 ~ 01327028 n 0000 ~ 01327322 n 0000 ~ 01328121 n 0000 ~ 01328302 n 0000 ~ 01383638 n 0000 ~ 01384313 n 0000 ~ 01384687 n 0000 ~ 01385527 n 0000 ~ 01386182 n 0000 ~ 01386354 n 0000 ~ 01415626 n 0000 ~ 01415920 n 0000 ~ 01416213 n 0000 %p 05220461 n 0000 %s 05267345 n 0000 -c 05431926 n 0000 -c 05432623 n 0000 -c 06088995 n 0000 ~ 07940242 n 0000 -c 08657249 n 0000 ~ 09819860 n 0000 ~ 10203839 n 0000 ~ 10297234 n 0000 ~ 10341660 n 0000 ~ 10456603 n 0000 ~ 10603959 n 0000 ~ 10648033 n 0000 ~ 10743675 n 0000 ~ 12992868 n 0000 ~ 13084479 n 0000 ~ 13084633 n 0000 ~ 13124164 n 0000 ~ 13124358 n 0000 ~ 13124529 n 0000 -c 13514314 n 0000 -c 13517553 n 0000 | a living thing that has (or can develop) the ability to act or function independently  
00005787 03 n 01 benthos 0 002 @ 00004475 n 0000 + 02661574 a 0103 | organisms (plants and animals) that live at or near the bottom of a sea  
"#;

// data.verb lines

const DATA_VERB: &str = r#"
02409148 41 v 02 overwork 0 exploit 0 008 @ 02407987 v 0000 + 01867768 a 0203 + 01867768 a 0201 + 01867768 a 0202 + 00948206 n 0201 + 00623370 n 0101 + 00623370 n 0102 ~ 02408530 v 0000 02 + 08 00 + 09 00 | work excessively hard; "he is exploiting the students"  
02409412 41 v 03 hire 0 engage 1 employ 0 014 + 13968092 n 0302 + 01217859 n 0301 + 10053808 n 0301 + 10054657 n 0301 + 01217859 n 0202 + 09867956 n 0102 ! 02402825 v 0102 ~ 00751775 v 0000 ~ 02394081 v 0000 ~ 02401399 v 0000 ~ 02409838 v 0000 ~ 02409941 v 0000 ~ 02413140 v 0000 ~ 02461063 v 0000 01 + 09 00 | engage or hire for work; "They hired two new secretaries in the department"; "How many people has she employed?"  
02409838 41 v 01 ship 0 002 @ 02409412 v 0000 + 04194289 n 0101 01 + 09 00 | hire for work on a ship  
02409941 41 v 04 sign 1 contract 0 sign_on 0 sign_up 0 004 @ 02409412 v 0000 + 06520944 n 0201 + 10597234 n 0101 ~ 02410719 v 0000 02 + 09 00 + 02 01 | engage by written agreement; "They signed two new pitchers for the next season"  
02410175 41 v 04 retain 1 continue 0 keep 2 keep_on 0 003 @ 02679530 v 0000 + 05051896 n 0203 $ 02747922 v 0000 02 + 09 00 + 10 00 | allow to remain in a place or position or maintain a property or feature; "We cannot continue several servants any longer"; "She retains a lawyer"; "The family's fortune waned and they could not keep their household staff"; "Our grant has run out and we cannot keep you on"; "We kept the work going as long as we could"; "She retained her composure"; "this garment retains its shape even after many washings"  
02410719 41 v 01 contract_out 0 001 @ 02409941 v 0000 03 + 02 00 + 08 00 + 21 00 | assign a job to someone outside one's own business  
02410855 41 v 02 work 0 do_work 0 021 $ 02407987 v 0000 $ 02413480 v 0000 $ 02594979 v 0000 + 00584367 n 0102 + 09632518 n 0101 ~ 01095218 v 0000 ~ 02343056 v 0000 ~ 02394183 v 0000 ~ 02408281 v 0000 ~ 02411529 v 0000 ~ 02411621 v 0000 ~ 02412939 v 0000 ~ 02419773 v 0000 ~ 02420232 v 0000 ~ 02420606 v 0000 ~ 02420789 v 0000 ~ 02420991 v 0000 ~ 02421199 v 0000 ~ 02460883 v 0000 ~ 02464481 v 0000 ~ 02576110 v 0000 02 + 02 00 + 22 00 | be employed; "Is your husband working again?"; "My wife never worked"; "Do you want to work after the age of 60?"; "She never did any work because she inherited a lot of money"; "She works as a waitress to put herself through college"  
02411529 41 v 01 tinker 0 001 @ 02410855 v 0000 01 + 02 00 | work as a tinker or tinkerer  
"#;

// index.adj lines

const INDEX_ADJ: &str = r#"
.22-caliber a 1 1 \ 1 0 03146310  
.22-calibre a 1 1 \ 1 0 03146310  
.22_caliber a 1 1 \ 1 0 03146310  
.22_calibre a 1 1 \ 1 0 03146310  
.38-caliber a 1 1 \ 1 0 03146602  
.38-calibre a 1 1 \ 1 0 03146602  
.38_caliber a 1 1 \ 1 0 03146602  
.38_calibre a 1 1 \ 1 0 03146602  
.45-caliber a 1 1 \ 1 0 03146895  
.45-calibre a 1 1 \ 1 0 03146895  
.45_caliber a 1 1 \ 1 0 03146895  
.45_calibre a 1 1 \ 1 0 03146895  
0 a 1 1 & 1 1 02186132  
1 a 1 1 & 1 1 02186338  
10 a 1 1 & 1 1 02187296  
10-membered a 1 1 & 1 0 01503760  
100 a 1 1 & 1 1 02196107  
1000 a 1 1 & 1 1 02198752  
1000th a 1 1 & 1 0 02212473  
100th a 1 1 & 1 0 02209423  
101 a 1 1 & 1 0 02196211  
101st a 1 1 & 1 0 02209551  
"#;

// index.adv lines

const INDEX_ADV: &str = r#"
'tween r 1 0 1 0 00250898  
'tween_decks r 1 0 1 0 00498293  
a.d. r 1 0 1 0 00001837  
a.k.a. r 1 0 1 0 00270446  
a.m. r 1 1 ; 1 0 00251304  
a_bit r 1 0 1 1 00033663  
a_cappella r 1 0 1 0 00001740  
a_fortiori r 1 0 1 1 00063483  
a_good_deal r 1 0 1 0 00059171  
a_great_deal r 2 0 2 1 00059171 00059413  
a_hundred_times r 1 0 1 1 00359932  
a_la_carte r 1 0 1 0 00257981  
a_la_mode r 1 0 1 0 00498182  
a_little r 1 0 1 1 00033663  
a_lot r 1 0 1 1 00059171  
"#;

// index.noun lines

const INDEX_NOUN: &str = r#"
abductor_muscle n 1 2 @ ~ 1 0 05291010  
abecedarian n 2 1 @ 2 0 09755788 08093653  
abecedarius n 1 1 @ 1 0 06377971  
abel n 2 2 @ ; 2 0 10807197 09587217  
abel_janszoon_tasman n 1 1 @ 1 0 11332068  
abel_tasman n 1 1 @ 1 0 11332068  
abelard n 1 1 @ 1 0 10807317  
abele n 1 1 @ 1 0 12732009  
abelia n 1 2 @ #m 1 0 12671651  
abelian_group n 1 1 @ 1 0 06017472  
abelmoschus n 1 3 @ #m %m 1 0 12171750  
abelmoschus_esculentus n 1 3 @ #m %p 1 0 12171966  
abelmoschus_moschatus n 1 2 @ #m 1 0 12172481  
abelmosk n 1 2 @ #m 1 0 12172481  
abenaki n 1 1 @ 1 0 09645871  
aberdare n 1 2 @ #p 1 0 08895148  
aberdeen n 4 3 @ #p + 4 0 09153570 09139380 09094093 08892186  
aberdeen_angus n 1 1 @ 1 0 02405929  
aberrance n 1 3 @ ~ + 1 0 14503665  
aberrancy n 1 3 @ ~ + 1 0 14503665  
aberrant n 1 2 @ + 1 0 09755893  
aberration n 3 3 @ ~ + 3 1 14503665 14386590 11420139  
abetalipoproteinemia n 1 1 @ 1 0 14151884  
abetment n 1 2 @ + 1 0 07251619  
abettal n 1 2 @ + 1 0 07251619  
"#;

// index.sense lines

const INDEX_SENSE: &str = r#"
abandonment%1:04:03:: 00204439 1 3
abarticulation%1:26:00:: 14294271 1 0
abase%2:37:00:: 01799794 1 0
abasement%1:04:00:: 00273449 2 0
abasement%1:26:00:: 14440623 1 1
abash%2:37:00:: 01792097 1 0
abashed%5:00:00:discomposed:00 00531628 1 1
abashment%1:12:00:: 07508092 1 0
abasia%1:26:00:: 14549070 1 0
abasia_trepidans%1:26:00:: 14549284 1 0
abasic%3:01:00:: 02598608 1 0
abatable%5:00:00:stoppable:00 02288022 1 0
abatable_nuisance%1:09:00:: 05830832 1 0
abate%2:30:00:: 00245059 2 0
abate%2:30:01:: 00245289 1 0
abatement%1:04:00:: 00361333 2 0
"#;

// index.verb lines

const INDEX_VERB: &str = r#"
abduce v 1 1 @ 1 0 01015866  
abduct v 2 5 ! @ ~ + ; 2 0 01471043 01449427  
aberrate v 2 3 @ + ; 2 0 02662076 02661769  
abet v 1 2 @ + 1 1 02549211  
abhor v 1 2 @ + 1 1 01774426  
abide v 2 5 @ ~ $ + ; 2 2 02637202 00668099  
abide_by v 2 3 @ ~ $ 2 1 02542280 02457233  
abjure v 1 2 @ + 1 0 00798717  
ablactate v 1 2 @ * 1 0 01186958  
ablate v 2 3 @ + ; 2 0 00275466 00177578  
abnegate v 3 3 @ $ + 3 0 02213074 01116275 00758042  
abolish v 1 3 ! ~ + 1 1 02427334  
abominate v 1 2 @ + 1 0 01774426  
abort v 3 4 @ ~ * + 3 0 00353839 00060063 00059899  
abound v 2 3 @ ^ + 2 1 02715279 02715595  
"#;
