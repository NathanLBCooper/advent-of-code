use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let sum: i32 = re
        .captures_iter(INPUT)
        .filter_map(|cap| {
            let n = cap.get(1)?.as_str().parse::<i32>().ok()?;
            let m = cap.get(2)?.as_str().parse::<i32>().ok()?;
            Some((n, m))
        })
        .map(|(n, m)| n * m)
        .sum();

    println!("Part 1 Sum: {:?}", sum);
}

const INPUT: &str = "?select()@ )select()>,how()mul(627,742)<??$&@mul(556,721)- [mul(436,900)who()^][*mul(339,267)<mul(932,131),%?mul(946,538)-who()mul(120,296)~mul(470,838)]:#<how()mul(582,167)mul(89,630)/:--select()how(715,850)/>:)mul(472,964)where()^^:]don't()-(,who()$select()mul(20,292)&why()-?mul(586,194)where()+[~@>?'mul(432,844)mul(642,543)where()[where():how(),select()mul(73,93)!what())!::@why()when()*mul(67,967)when()~)(mul(63,791):>from(),how() %(mul(843,987)+<*mul(438,132)>&@why()mul(460,493)where()where(524,51)mul(59,82)&mul(989,617) ?>[when(),mul(504,567))where()-'/where()why()mul(426,401):#{mul(776,395)>'?)}'mul(898,813)%from()~;[/mul(653,852)mul(413,308)'/mul(938,447)~,>when()select()mul(983,694)!select()$!,->mul(475,864)mul(396,963)what()select(){][:/&@mul(154,218)how()<&mul(538,894)^-~[who()mul(962,146) mul(847,87)where()do()<?'~ how()+}*'mul(383,724)''*[ mul(391,862)>~ ~[select()mul(900,385)mul(943,471)@select()[from()^mul(283,141) ++>mul(52,893)# do()mul(449,81)*where()(<mul(87,134)#why()?don't(), -mul(905,619)who()((mul(211,136)what()&&?{;select()!what(38,53)mul(926,865)how()/%mul(317,285)(?^,-mul(450,990)from(269,194)#{*why()from()why()-<mul(388,169)$&,'do()@?when()%}~mul(481,655)!$'{mul(285,947) }mul(763,507)<mul(777,596)-:'why()}what()mul(411,654)where()mul(808,376)[*$do())/^mul(365,558):^:when()(~'select()-mul(797,507)when()what()};+mul(882,986)why()^why()(]~(mul(894,948)mul(65,514))~<what()mul(755,681)~*/where()@mul(47,331){&mul(519,725) {mul(389,822)?why()!{when()when()+where()mul(888,943)how(448,140)@+^'mul(426,458)<mul(668,193)mul(466,81)] ,/!['(mul(831,631)&->mul(548,611) /from()&where()why()^from()mul(865,16)@&]%:{^mul(762,99):/how()&{mulwhen()>when();)}]#mul(565,345)where()from(),&why()mul(438,239)where()(>({mul(368,412)#:<@ }~<mul(935,813)~mul(310,194)+mul(785,553)mul(566,215){from()!,{/(mul(318,50)/select()$*how() ^<]mul(104,155)mul(840,970)?>mul(837,348)&mul(56,613)]  how()/ :+mul(720,439)mul(459,291)@-& mul(168,52)^>+don't()mul(453,478)^<-+when()who() *?from()mul(741,484)mul(975,310),how()*?mul(127,418)&where()mul(185,669)$select()^#mul(288,189%why()when()do()@how()+/~who()what()(mul(784,991)^<what()mul(969,971)[{what()mul(436,579)+$^],;$what()'>don't();'!&>$mul(748,589)who(){{select()mul(539,888)):)$>mul(650,904)^mul(11,905)$when())mul(3,778)(?from()-]>mul(216,342)()mul(678;don't()'%]:mul(297,302)mul(356,884)mul(915,227)]~*[!:from();@@don't()mul(701,800){when()mul(747,577)when()^!select();[#)[<mul(389,544);##%:)select()}who()select()mul(129,285)[<!/where()~(select()where()mul(899,32)why()+when()why()select()-why()~$<mul(804,862)#(}mul(612,164),*$*}why()&&mul(512,913)}>where()from()mul(813,504)how()how()>}mul(569,829)what()!%&mul(91,228)[where(764,748))from()mul(475,767)(mul(975,908)${-why(43,497)]$why(),>mul(408,155)!select()>$##when(549,859)mul(906,726)@[' don't()from()}when()^mul(811,889)
]};don't(){^mul(131,421)who()+where()why()why() mul)who()from(361,208)#($>/mul(986,7)~!/+:what(911,564)&~mul(427,317):<how()[-+?from()*do()??$'why(),$#,(mul(388,863)]$;mul(93,214)${from(),>mul(554,29);when(),@#who()),mul(203,377),when()<%;[%;mul(459,428)$where()mul(289,903)$;?]what(),when()%*mul(920,908)~%from()>)?!++@mul(328)-/from()[['[[:-mul(149,962):~((select()#)@~,mul(405,65)$select(),^'/select()mul(242,181)mul(286,414)!mul(380,335)>:select()'!',from()mul(566,10)(,when()who()mul(859,933)<~,why()]#[(,mul(563,784)select()}mul(184,412)[mul(123 :<+((:}why()@[mul(368,888)what(),:mul(902,504)who()!mul(416,510)}~<mul(625,533)+how(9,916)[who()when()mul(849,200)]]?)(mul(926,585)#mul(799,213)$(what()!*@~#?mul(807,743)>select()#$<mul(805,251)when() @<when()/}mul(673,225)how()mul(845,963)}]mul(70,534)[$^how()mul(879,88)]?how()[where()where(634,718)~don't()>who()#$mul(47,446)#!%>mul(781,779)select(),where()$mul(321,388){]who(988,158)@select()$)!what()mul(461,690)mul(250,858)];{-where()mul[%&+{mul(464,42)where(693,854)&/{from();/mul(276,502)when()when()why()select())from()mul(741,208)}<mul(667,677)mul(945,856);!-<from()>do()when() mul(393,259)<} do()&'why()/when(),?who(167,431)mulfrom();how()where()select()%^from()%$mul(178,393)/why()!%@+how())'mul(40,701)[$/)*:{mul(543,32)! +from(),how()(~mul(279,463)where()%mul(140,229)[;]when()!( mul(677,59)mul(299,328)what(){from()#who()mul(685,444)*{>when()~who())mul(785,608)where()(}mul(401,742)$-mul(548,663);-from()from()/@)why()$mul(607,432);~>@*+*!,;don't()}@;]when())^mul(709,472)*]{where()when(591,697)mul(358,548)how()when()what()@@+mul(213,262why()~&#why(47,493)don't()[mul(627,121),who())-mul(593,802)who()!from(547,353)%select()select()mul(535,564)/)*}^~from()mul(88,940)-#}*$}don't()mul(618,335)^why()from()@mul(803,303)where()<why():@where();<>where()mul(587,871)what()! how()~'mul(331,145)?#how(204,874)why()&when()mul(499,131)how()#%+what()mul(161,858)why()mul(898,747)mul(485,905)-~why()/}mul(387,741)-*([mul(786,326)(what()mul(845,292)~select()mul(381,537)what()select()?select(): $[mul(48,610)~]}who()select()when()<]mul(347+why()*$%mul(115,487);+mul(190,645)*who(356,447)?$$who(560,786)mul(269,713)~$>>:{^(^mul(405,453),~what(956,701)@mul(714,736)how(571,733)select()+}~#mul(150,985){,[!@-'[-{mul(306,321)what()-+~#)mul'#?select()mul(46,828)^^]>,>mul(990,982)@mul(226,149)${&)<mul(66,604):]% why()from()~:mul&how()$)why()[what()}mul(137,237)@:how(),;] *>mul(280,466)&[;what()from()why()who();~why()mul(721,24)][,]mul(278,485);)select()how()what()mul(647,743)who()%where() /mul(133,662)what():}%mul(145,101)who(){mul(452,439)%when()where():;%[ from()mul(43,332)#)mul(814,129)~select()}>*~<what()->mul(541,874)mul(15,125))who()$from()do()select()~who()[@~-#mul(831,715)~when()*what()mul(105,955)mul(578,337)?what()[:'/what()what();select()mul(409,86-don't(),[<*mul(368,215)>+mul(574'+{}where(495,68)select(64,771)who()]?mul(529,984)'mul(327,862)'+~@why()why(){)*<mul(157,75)^why()@mul(374^mul(171,938)+:where()-]mul(296,833)^select()<mul(522,120){select()how(){,mul(504,18)~mul(815,828)<}when()mul(376,713)(%mul(313,724)[mul(875,924)!select()?%,)*>&mul(550,427)]'&*%/*mul(263,618)@who(540,556);why()where()/how(995,745)mul&who()mul(915,810){select()$]why()]?@#$mul(539,921)?}/#mul(998,491){%+^%(mul(225,651)}:!who()@]$mul(389,253)]<!what()~who()*%mul(728,341)%%select()'mul(654,870){?^-^?do()'mul(743,813)
,what()(?,-$<mul(478,167!@who()-mul(675,819)^do()^}from()%)/,mul(947,378)) +}>['mul(522,712)#>^&[*what()mul(94,532)&when()@where()(mul(59,866)-what()>,{%mul(699,198)~:;#+ -mul(336,760)where()+select()/&mul(751,64)-[-mul(643,199)('(*:< why()%mul(11,372)where()#mul(413,24)what()^${%why()when()]mul(73,17)@mul(243,731):select()#'^%^;@mul(368,517){:{;]$who(599,355)#-mul(532,854)+',what()what()*+don't()mul(283,334)~%{!>$:mul(5,436)who()-mul(900,533)[how()mul(636,366)how()]mul(777,720):mul(508,874)<-+why()&!from()mul(9,795)'#(mul(671,121)where()@;(how()mul(585,980)-{{>>*[:from()mul(884,247)where()how()when()mul(43,381)mul(711,197)*from()where(953,507)^!mul(410,514)select()-how()<:mul(183([!)+what()}who(735,345)~':mul(158,233)how())from()don't()@,>mul(192,433)[ mul(486,375)}&[how()~/(%mul(737,191)how()!mul(200,635)! ;mul(589,747)who()<?,!how()$mul(54,885),&~%!do()+'who()/how()]:mul(920,155:where()mul(636,189),(<,who()(:mul(779,355)^mul(20,937),,!mul(579,757)};,;{/@mul(693,103)')#*what()(when(956,598)don't()where()#^why(),]how()*>:mul(93,243)mul(165,137)&(,< select(646,92)[-mul(304,19^,~what()#>:%}+mul(714,141)%,;mul(352,220)mul(731,110)why(939,116):when()$[}'$)mul(499,210)*from()>mul(478,440)#when()when()why()&}from():from()mul(364,337)'*]{where()%%mul(137,747)mul(123,534)mul(269,530)mul(976,777)why()$why()where()+{mul(477,911)mul(264,388)>do()where()from(342,309)(*from()-mul(363,449)/~}mul(622,492)mul(828,345)?~why(565,199)what()}#)!{%mul(895,692)mul(301,191),(when()'when()! mul(203,800)&/$,#/where(84,417)mul(366,29)[/from()[>)from(),where()&mul(624,385)--]@]where()]@mul(207,15)mul(912,159)why()(%?#mul(734,622)]?~)who()!]mul(732,406)[+{mul(924,240)?from()/mul(613,858)]-[how()#{mul(981,919)mul(62,846)/mul(194,383)what()@>}~[mul(48,80)mul(517,599)*$select()&(#@;'+mul(400,485) }{%-<(!-mul(159,943)'-mul(700,581how()from()mul(335,789)(mul(884,411)[&mul(591,291)how():who() ^select()mul(819,468)select()/,$select()mul(391,484);(![mul(121,429)/?<mul(348,41)@<where()from()where()~{[don't():/select()@when()mul(335,34)mul(498,153)who()how()(from()@why();mul(864,694)who() '!mul(331,252)how()where()where()}why(497,689)why()?what()why()mul(174,337)/<who()$when()?who(47,26)where()from()mul(178,783){'$-mul(418,277)$/[^mul(156,213)why()>!mul(41,994)#mul(533,286),?@mul(683,23)select(),?;mul(716,809)&^@[*]* mul(186,301)^&^}/mul(665,90)} [$,%;*}:mul')<<]mul(969,145){mul(409,661)#mul(425,476)'mul(845,137)[why()!<mul(271,724)*[!from()[why()[) 'mul(231,976)mul(718,828)~mul(241,249)@#>mul(54,557)mul(509,846)@:*do()^>[+/when(),mul(863,394)when()#^what()who()%why()]mul(471,711)from()@mul(748,203)!'-where()why()(]how()mul(337,399)why()?$$&;mul(973,320)>who()select()/mul(584,242):who()%*+]when()^mul(579,798)'@mul(644,368)when(991,226)@&+*[?$mul(926,437)('[who()who()select()where() <mul(908,692)%{%!mul(46,304):~;+-!@$,<mul(976,157)mul(256,67)[^>?mul(533,162)*what()??@&!/,from()mul(491,941)}select()from()mul(366,787)
#!/usr/bin/perlmul(959,994)[mulwhere(315,563)^,@ *![)+?don't()(*#select();&&(why()mul(868,148)-)*'mul(803,674):what();-~{^<,<mul(940,379)select()why()+ }]where()who()mul(65(mul(983,309)from();+mul(214,888)^mul(446,409)@$>(}how()+(,mul(241,227) $how()mul(77,823)<:@@~mul(899,241)where()/[%!mul(393,869)mul(807,502)select()?$select(628,387)*mul(266,866)$from()+mul(643#mul(359,341):~>(*%select()>mul(48,678)'from(104,180) do()what()/why();what()mul(209#from(754,77):where()(}! mul(482,838)]~when():?:mul(144,520)mul(214,696)when()~+:what(629,799)mul(586,883)!&*mul(513,456)#*&mul(231,739)^@from()mul(314,88)&[[how()+$who()+mul(146,67)why()(&mul(491,624)!~%mul(404,364)how()/select(765,975)mul(996,512)~}]}?':from()$+mul(888,773)~why()!-%]?$who()mul(448,425)-;'*'when()'}@mul(699,626)mul(458,356)@:!*)select(679,5)-mul(921,391)select(86,936)%}'*select()mul(139,417){: mul };from()+%mul(21,568)from() mul(859,475)from(57,335)%from()&(@-mul(458,265)from()from(58,643)when()?*('$-mul(922,170)select()*<!~><~{*mul(343,183),mul(993,879)where()mul(988,140)from()-;?}+mul(119,739){why()+*{!mul(942,337)who()~mul(289,885/how()*;<mul(569,577)don't()?'!]^>')why(651,68)mul(52,645):/&when()/how()select()mul(301,938){(~(>]where()-mul(713,126)@mul(840,862)&why()?mul(138,715)#@}mul(479,429)?>? }:'-when()mul(680,926where()')<@'when()how()who()<[mul(847,417) *)who() mul(936,706)'when(881,614)#[)([{select()[mul(355,109)#who(454,930)#mul(886,700)@;)who()how()&,&mul(39,902)who()mul(780,180) ]who(253,678)%mul(787,109),&) ${^mul(978,343)<@{/why()mul(506,765)&@[;'mul(709,534)mul(17,379);*how()#][] when()where()mul(738,857)<how():,how()mul(122,507)where()from()-(@<where()&mul(447,689)$,~$/;(&(mul(270,940))!%>what()(/select(),'mul(472,481)%<$ 'mul(956,729);when()?[,,when(846,880):]mul(290,627)]mul(505,77) ,how()!>-from()mul(416,570)how()%mul(911,609)}?how()? %~]select()mul(195,598)when()^}where()}do()]select()what()(['<]what()mul(475,752)$mul(538,26)},+,mul(329,974):&where()why()%mul(611,429){?[<mul(122,473)%,where()-)how()@>mul(122,920)why():&mul(98,164:why()why(),!')*%mul(108,605)~,how()#)'how()how()%mul(350,554)])(^;mul(221,276)](*select()when()from()-mul(879,301)when()& &)<mul(259,810){mul(572,185)*what()do();!when();>]mul(866,108)$when()]?mul(814,972)<-,)++!from()what()mul(138,653)select()+{}#-::#mul(260,901))-~+{why()how()>how()mul(271,687)mul(260,481)#>/select(881,820)<[/>mul(340,4)>:$where()what()mul(192,605)where()mul(542,534(!$,mul(269,337)mul(693,324){*)]:what()>@)mul(995,639)?[,who(),/{ ?@do()%%mul(197,198)from()when()!&[&don't()who():-select()?&when()$-mul(833,76@select()mul(769,973)!(}don't()mul(886,148)mul(290,734)~-mul(363,635 !{select()where()!select()<'where(335,974)mul(407,315)!+how()& &)mul(114,97)'<who()?+~&;!mul(618,180)?>mul(310,507)]<mul(546,823)mul(336,915)'where()mul(475/select()~?who()>{@'':mul(518,510)%+}where()]*mul(807,827))*mul(983,153)who()$mul(992,402)'/?from()mul(432,440)^$-select()*do()mul(859,550),/when(),-mul(571,66)#:from()&'why()when()^don't()where()<what()#mul(948,293)]-?what()*mul(88,418)what()~{{~:,--;mul(964,366)#}-(#mul(549,708)[++mul&[?@/-,)why()when()mul(499,377)how(438,944)~[mul(894,734)-#< :mul(130,348)mul(977,723)@<^])don't()!how()where()mul(397,197)^%where(346,589)*[what()mul(214,862)< >*mul(615,334)who()@+!!}mulwhat(40,947)select()mul(74,725)
mul(325,134)][mul(91,563)mul(624&(<)mul(880,271):]@/when()*from(); mul(433,630)where()why()#from()-~;-mul(357,578)@('don't()where() ^{:,mul(727,465)how()*}>who(){-mul(452,304)why()-})'/%?{ mul(884,733)$>@['@/mul(211,96)select()'&+#&what()mul(184,747)what()++(mul(645,340)where(){'when()mulwho(965,444)]{(mul(843,881)+/,-()%how(874,562)#do()+'{mul(64,698){from()&/mul(140,563)';)do()'*;  [),mul:from()what()why()~!%,where()}when()mul(40,687)when()select(600,672)?$>who(853,252)/&:)mul(527@%+>*%!when()+mul(568,99)#^why()]'^)select()mul(920]( what()what(),<:;@mul(809,803)from()@mul(405,306)}]]~~-<(mul+why()% '',:+}$mul(478,156),{?-mul(554,837)#,from()how()when()@'who()mul(229,278)]}%[)}what()-mul(761,692)(#:<''$mul(655,602)@from()'select()mul(156,419)why()-when()mul(794,965)+why()*why()]!from(){!do()from()why()when()where(172,132);?}-mul(329,393)from()$&what()mul(640,220)]^&select()[!who()@mul(784,552)mul(173,583)what()what()why()+;from()$mul(869,282):~}>}mul(932,355)%,+why(){(*]>#mul(147,179']@<&&^:do()'-mul(256,778)+how()+select()&+(do()(,~mul(695,295)how()~%~/&>select()%*don't()~]how()^%~{,'mul(642,510)select()$/why()(%;&?'mul(38,506)&/!;mul(721,659){!why()~?!mul(580,780)what()from()]-how(){mul(961,23!from()%$]@mul(716,597):?where(838,897)( how()>?mul(440,697)>,,~?//mul(876,363)why()[+how();!from()mul(95,795)@mul(507,198)@}don't()+ /-mul(219,968)<mul(597,684)]/?&don't()why()~/why()!where()how()<select()mul(350,634)^/~how()#;where(){(?mul(927,790),mul(963,990)[who()how()+>(]do()+select()when()[)!]mul(193,8)/)do()mul(614,596)when()-%;]where():-}mul(683,732):*<,mul(934,88)from()[from()])mul(943,457)}?when(263,413)$;why()~where()don't()[how()how()who()from()/mul(526,391)!?from(){>mul(771,644)}}who()$where(),! mul(110,448){where()~mul(921,576)@-mul(330,793)*]+-*#/mul(660,134)why() :#?('*]'mul(130,438)what()~+select()}why()?/{$mul(748,334)<)]who()''mul(58,989)how()@mul(994,89)why()'do()>?:-(how(771,295)mul(298 '}{who()mul(960,353)+]mul(83,238)'&$^+,mul(304,737)mul(580,87))[*why()select()mul(243,740)^%$:-'^don't()-mul(466,277)from()mul(697,988)from()how()why()^ why()>mul(656,652)who()~why()select())mul(559,696)?mul(877,396)when(921,105)>(;mul(137,506)#from()from():>+/mul(529,400)mul(357,762)/'-mul(957,435)]#^+-&#mul(965,429)) {!)?mul(472,501)*how()(select()##mul(452,510)mul(685{&#-$what()mul(597,100)([['%@$mul(381,890)what()%$~->why()--mul(717,973)select()# why()mul(335,280)[^,)]@]&when()?mul(998,855){how()how()+/+mul(803,232)(@)%,where()mul(834,38)#mul(874,900):where()who()}'where()mul(829,368)[(< (- mul(285?~who(240,567)mul(564,804)what()mul(843^{())$from()where()what()%mul(993,383)why()>select()select()- */don't()who(775,71)from()[mul(462,424'>)~when()-,,&*mul(567,875)+mul(851,776)when()]?}what();mul(675,128)/!~-how()~;from()mul(561,378) ^~mul(851,971)how()^#:' mul(665,836)$mul(972,115 ,)mul(83,568)$(?how()(< ;'/mul(252,421)*where():who()!@,mul(516,23)%!mul(573,894)*why()^[select()*who(),mul(806,961)how()!(what() mul(141,808)>@-%;mul(90,355)who()mul(35,600):+mul(94,467)&mul(259,492)'!from()~mul;why()/[mul(745,850)#*%where()&::*>'mul(103,841)*?~&,:(mul(612,663)what()[mul(377,477))how()%-}:where()don't()why()what()(where()(mul(545,124)(/$who()$do()mul(466,334)@what(123,251):+mul(441,792)}?^<[*#mul(190,892)'+*$#mul(544,237)select()$:>mul(326,530)'+)*,mul(872,149)why()}&mul(269,670)mul(773,795)why(168,560)when()@]&#))?mul(7,32)-)where()mul(59,123)&who()}<')$mul(668,284!>mul(820,879)
mul(384,175)?$:how()&mul#mul(313,175)>where()!from(300,354),mul(408,50)mul(466,64)%?^mul(228,465)+where()mul(397,508'%where()from(530,982)^~mul(57,161)!?where(817,534))($how()from()select()(mul(807,622)!mul(295,207)who()++*when()]:when()mul(565,911):select()what():;mul(812,870)#/%[@!<>>,mul(53,989)[[!(mul(294,143)from()when(){how()(@don't():when()where()/select()mul(926,422)what()mul(978,922)<~mul(37,714)%mul(6,177)why()[ <why() *[ !mul(38,923)?!<what()do()])who()mul(2,539)^mul(929,157)mul(169,548)&mul(234,619)who()who(),@why(592,458)[mul(437,685)/+where()<@mul(879,577)()(}(who()}mul(978,147):}where();don't()${<$mul(610,235)/$;where()}[where()mul(476,748)@where(578,824) '!'mul(453,691)*(+^;when()what()#{mul(143,495)#what()-'}do()@-why()#!>?from(881,644)mul(409,551)mul(527,110)from()&mul(775,908)select() where()select()&>:mul(756(%/how()&)do()<,>mul(960,35);%'((don't()*[~&(&:)mul(397,932)'*%$/[mul(413,801)*& from()+why()mul(744,321){]mul(478,898)%what()~select()/when()*/mul(664,447)select():;?(mul(557,407)/how()when()*mul(487,654)> -^?;when()mul(580,577)select()mul(328,532))]>$mul(562,446)-{from(271,895)!,[who()don't()what()%mul(170,505)why()what()> mul(547,823)mul(599,747))<!<~#when(13,741)<)do()when()what() -)<+where()#,mul(122,641)how() how();/mul(851,215)(mul(311,265)-//]#mul(602,408)}]<$select()~>!mul(923,219) !]&])why()how()mul(343,142)mul(166,723)who()(&when()how() why()}[]mul(356,537)don't()[%from()from()how()from();$,mul(298,224) }where()*mul(603,208);what()from()who()]mul(110,59)-<{: :mul(830,39)'['*#-$@mul(53,946)#}[how(75,668)%[what()*]who()mul(439,241)[^^>don't(),}from()(>where(112,444)why()when()mul(42,810)mul(732,408)#!;mul(43,449)%why()?(#@%do():mul(454,178)(when()>+'?{^ mul(576,351)?who(219,159)><when()from()(&why()<mul(623,693)@~/#%)->$$do()^*~why()?@what()?mul(425,748)!;'mul(99,347),){%%)$how()what()}mul(63,113)select(460,662),%(!/-';mul(921,332)#mul(850,246),mul(18,318))from()[~mul(984,78);'&@]@}mul(217,578)/${&??)#')mul(480,37)how()mul(993,178)#';]select()~{mul(66,840)?when()who(941,421)'mul(958,953),}/&mul(893,645)(mul(669,561)mul(932,433)why()^)mul(129,67)$/mul(731,199)how()]>from()(:/)mul(664,507)[)/mul(282,709)who(),select(998,269)when(683,65)mul(131,8)'})>%mul(816,219)why(){when(6,871)'+mul(802,853));:mul(252,355)?select()!-don't())::what()mul(670,441))[mul(591,136),@ mul(338,232)@}mul(944,594)@what()}~>:what(596,921)mul(121,669)%}+mul(306,982)#^from()mul(530,368)#select(){!;!+!what()mul(809,843)%mul(997,156)^'where()$how()mul(444,325)}mul(14,843)from(989,984)#(<< +when()mul(330,71)*mul(277,727)who()(&$#-do()'mul(386,736)who()<>*#)[where()mul(216,642),~<>how()@*mul(161,738)??)what()mul(328,349)(who()*why()?<>how()mul(158,187)^]'-mul(27,774)select()?where()mul(496,623)why()@~@select()who()?#mul(255,135)>%^do()!@why(),when(){@;:)mul(656,556))@who()[why()how();^;mul(46,946)&],+mul(956,617)select()'mul(858,593)how())$where()[~^when()mul(232,243),}~[ from()mul(403,657)[,:!>#{mul(617,333)mul(775,392)mul(773,876)what()'what() }(]&)[mul(438,813)^: $select()mul(355,337) )why()+how()?don't()~*when()mul(163,891)who(){[mul(521,763)what()";
