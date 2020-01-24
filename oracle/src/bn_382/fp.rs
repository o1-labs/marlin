use crate::poseidon::ArithmeticSpongeParams;
use algebra::fields::bn_382::fp::Fp;

use std::str::FromStr;

pub fn params() -> ArithmeticSpongeParams<Fp> {
    ArithmeticSpongeParams {
        round_constants:
      vec![ vec![ Fp::from_str("78119860594733808983474265082430117124674905785489385612351809573030163625517").unwrap()
          , Fp::from_str("41917899842730241418346215913324270532073353586134123463219061327941260175271").unwrap()
          , Fp::from_str("74594641694171623328644944059182600919855574964222988275913344198970402906473").unwrap()
         ]
       , vec![ Fp::from_str("96215759378377024990520153908983544755208851791126218239402755616994541522004").unwrap()
          , Fp::from_str("64070601581278917442704840630680311036021557676765751754522901046069205253111").unwrap()
          , Fp::from_str("112123228532462696722378911494343451272980413618911326680094528285518792872677").unwrap()
         ]
       , vec![ Fp::from_str("84572244072021308337360477634782636535511175281144388234379224309078196768262").unwrap()
          , Fp::from_str("45201095631123410354816854701250642083197167601967427301389500806815426216645").unwrap()
          , Fp::from_str("23419302413627434057960523568681421397183896397903197013759822219271473949448").unwrap()
         ]
       , vec![ Fp::from_str("63220724218126871510891512179599337793645245415246618202146262033908228783613").unwrap()
          , Fp::from_str("67900966560828272306360950341997532094196196655192755442359232962244590070115").unwrap()
          , Fp::from_str("56382132371728071364028077587343004835658613510701494793375685201885283260755").unwrap()
         ]
       , vec![ Fp::from_str("80317852656339951095312898663286716255545986714650554749917139819628941702909").unwrap()
          , Fp::from_str("110977183257428423540294096816813859894739618561444416996538397449475628658639").unwrap()
          , Fp::from_str("25195781166503180938390820610484311038421647727795615447439501669639084690800").unwrap()
         ]
       , vec![ Fp::from_str("108664438541952156416331885221418851366456449596370568350972106298760717710264").unwrap()
          , Fp::from_str("17649294376560630922417546944777537620537408190408066211453084495108565929366").unwrap()
          , Fp::from_str("95236435002924956844837407534938226368352771792739587594037613075251645052212").unwrap()
         ]
       , vec![ Fp::from_str("43150472723422600689013423057826322506171125106415122422656432973040257528684").unwrap()
          , Fp::from_str("77355911134402286174761911573353899889837132781450260391484427670446862700214").unwrap()
          , Fp::from_str("8690728446593494554377477996892461126663797704587025899930929227865493269824").unwrap()
         ]
       , vec![ Fp::from_str("109175231986025180460846040078523879514558355792739714578031829643740609438879").unwrap()
          , Fp::from_str("64844253590731404811389281562033735091759746904073461140427127388042062490899").unwrap()
          , Fp::from_str("43237071281695629980341250188156848876595681601471702180515324064382368960951").unwrap()
         ]
       , vec![ Fp::from_str("2704440995725305992776846806711930876273040749514871232837487081811513368296").unwrap()
          , Fp::from_str("66806779110388532101035294912010606217442229808784290357894909707660045365269").unwrap()
          , Fp::from_str("25541187612624070470730890200174075890643652797181103367956318438136878170352").unwrap()
         ]
       , vec![ Fp::from_str("89300613074831725721350087269266903129165086877175223066581882601662278010666").unwrap()
          , Fp::from_str("36824076981866281177052433916337787028520068526782493484076995129329938182524").unwrap()
          , Fp::from_str("68880449342008497744225106025198236600142055580985632884415488154606462819445").unwrap()
         ]
       , vec![ Fp::from_str("68556888546596545408135887526582256648006271867854316538090068824142539400698").unwrap()
          , Fp::from_str("111379753250206255125320675615931203940253796355491142745969887430259465111569").unwrap()
          , Fp::from_str("101469186248899356416491489235841069222521093012237305521090058066171355672289").unwrap()
         ]
       , vec![ Fp::from_str("87819793263125973233157093200229218382531712066157093399606059493857616731410").unwrap()
          , Fp::from_str("11055386921184594780372263378420826851562920740321950336882051897732501262543").unwrap()
          , Fp::from_str("111945832089295501567161822264292548657346358707472584179854375613919325491249").unwrap()
         ]
       , vec![ Fp::from_str("95630018375719472826904441325138673248990446382783206900295723762884876505178").unwrap()
          , Fp::from_str("94833984285990985873155989049880754188702918168949640563745233736765833491756").unwrap()
          , Fp::from_str("77578854197021606645372788474039811639438242484066959482386065023999206730771").unwrap()
         ]
       , vec![ Fp::from_str("27799616729223271646690718201487403976485619375555391888533887467404804041014").unwrap()
          , Fp::from_str("42616502170265664498961018686434252976977548128285781725227341660941880774718").unwrap()
          , Fp::from_str("95884094505080541517768389956970969462501217028562326732054532092615835087122").unwrap()
         ]
       , vec![ Fp::from_str("107531500891040898338150732759493933154418374543568088749403053559827078391994").unwrap()
          , Fp::from_str("17316158269457914256007584527534747738658973027567786054549020564540952112346").unwrap()
          , Fp::from_str("51624680144452294805663893795879183520785046924484587034566439599591446246116").unwrap()
         ]
       , vec![ Fp::from_str("17698087730709566968258013675219881840614043344609152682517330801348583470562").unwrap()
          , Fp::from_str("111925747861248746962567200879629070277886617811519137515553806421564944666811").unwrap()
          , Fp::from_str("57148554624730554436721083599187229462914514696466218614205595953570212881615").unwrap()
         ]
       , vec![ Fp::from_str("92002976914130835490768248031171915767210477082066266868807636677032557847243").unwrap()
          , Fp::from_str("58807951133460826577955909810426403194149348045831674376120801431489918282349").unwrap()
          , Fp::from_str("93581873597000319446791963913210464830992618681307774190204379970955657554666").unwrap()
         ]
       , vec![ Fp::from_str("46734218328816451470118898692627799522173317355773128175090189234250221977353").unwrap()
          , Fp::from_str("12565476532112137808460978474958060441970941349010371267577877299656634907765").unwrap()
          , Fp::from_str("54284813390357004119220859882274190703294683700710665367594256039714984623777").unwrap()
         ]
       , vec![ Fp::from_str("92046423253202913319296401122133532555630886766139313429473309376931112550800").unwrap()
          , Fp::from_str("15095408309586969968044201398966210357547906905122453139947200130015688526573").unwrap()
          , Fp::from_str("76483858663950700865536712701042004661599554591777656961315837882956812689085").unwrap()
         ]
       , vec![ Fp::from_str("37793510665854947576525000802927849210746292216845467892500370179796223909690").unwrap()
          , Fp::from_str("84954934523349224038508216623641462700694917568481430996824733443763638196693").unwrap()
          , Fp::from_str("81116649005575743294029244339854405387811058321603450814032274416116019472096").unwrap()
         ]
       , vec![ Fp::from_str("28313841745366368076212445154871968929195537523489133192784916081223753077949").unwrap()
          , Fp::from_str("17307716513182567320564075539526480893558355908652993731441220999922946005081").unwrap()
          , Fp::from_str("63148771170858502457695904149048034226689843239981287723002468627916462842625").unwrap()
         ]
       , vec![ Fp::from_str("14724939606645168531546334343600232253284320276481307778787768813885931648950").unwrap()
          , Fp::from_str("4684996260500305121238590806572541849891754312215139285622888510153705963000").unwrap()
          , Fp::from_str("63682763879011752475568476861367553456179860221069473817315669232908763409259").unwrap()
         ]
       , vec![ Fp::from_str("47776179656187399887062096850541192680190218704758942820514561435612697426715").unwrap()
          , Fp::from_str("42017618175533328439486588850450028995049195954365035474995309904751824054581").unwrap()
          , Fp::from_str("39169739448648613641258102792190571431737464735838931948313779997907435855102").unwrap()
         ]
       , vec![ Fp::from_str("37525991163523321662699819448962967746703579202577998445997476955224037837979").unwrap()
          , Fp::from_str("67759173441312327668891803222741396828094999063019622301649400178376863820046").unwrap()
          , Fp::from_str("23041132473771739182071223620364590606653086905326129708428084432335332411661").unwrap()
         ]
       , vec![ Fp::from_str("77778894465896892167598828497939467663479992533052348475467490972714790615441").unwrap()
          , Fp::from_str("20821227542001445006023346122554483849065713580779858784021328359824080462519").unwrap()
          , Fp::from_str("47217242463811495777303984778653549585537750303740616187093690846833142245039").unwrap()
         ]
       , vec![ Fp::from_str("42826871300142174590405062658305130206548405024021455479047593769907201224399").unwrap()
          , Fp::from_str("8850081254230234130482383430433176873344633494243110112848647064077741649744").unwrap()
          , Fp::from_str("1819639941546179668398979507053724449231350395599747300736218202072168364980").unwrap()
         ]
       , vec![ Fp::from_str("21219092773772827667886204262476112905428217689703647484316763603169544906986").unwrap()
          , Fp::from_str("35036730416829620763976972888493029852952403098232484869595671405553221294746").unwrap()
          , Fp::from_str("35487050610902505183766069070898136230610758743267437784506875078109148276407").unwrap()
         ]
       , vec![ Fp::from_str("62560813042054697786535634928462520639989597995560367915904328183428481834648").unwrap()
          , Fp::from_str("112205708104999693686115882430330200785082630634036862526175634736046083007596").unwrap()
          , Fp::from_str("109084747126382177842005646092084591250172358815974554434100716599544229364287").unwrap()
         ]
       , vec![ Fp::from_str("63740884245554590221521941789197287379354311786803164550686696984009448418872").unwrap()
          , Fp::from_str("58779928727649398559174292364061339806256990859940639552881479945324304668069").unwrap()
          , Fp::from_str("20614241966717622390914334053622572167995367802051836931454426877074875942253").unwrap()
         ]
       , vec![ Fp::from_str("41621411615229558798583846330993607380846912281220890296433013153854774573504").unwrap()
          , Fp::from_str("20530621481603446397085836296967350209890164029268319619481535419199429275412").unwrap()
          , Fp::from_str("99914592017824500091708233310179001698739309503141229228952777264267035511439").unwrap()
         ]
       , vec![ Fp::from_str("9497854724940806346676139162466690071592872530638144182764466319052293463165").unwrap()
          , Fp::from_str("7549205476288061047040852944548942878112823732145584918107208536541712726277").unwrap()
          , Fp::from_str("30898915730863004722886730649661235919513859500318540107289237568593577554645").unwrap()
         ]
       , vec![ Fp::from_str("22697249754607337581727259086359907309326296469394183645633378468855554942575").unwrap()
          , Fp::from_str("72771100592475003378969523202338527077495914171905204927442739996373603143216").unwrap()
          , Fp::from_str("84509851995167666169868678185342549983568150803791023831909660012392522615426").unwrap()
         ]
       , vec![ Fp::from_str("36601166816771446688370845080961015541431660429079281633209182736773260407536").unwrap()
          , Fp::from_str("19555759172327736128240171000715903945570888389700763573790859521156095228287").unwrap()
          , Fp::from_str("82844424532983875300577689116331373756526403900340445449185486212503235782229").unwrap()
         ]
       , vec![ Fp::from_str("40833119728631657038301474658571416779079199343770917422783737091842927892625").unwrap()
          , Fp::from_str("68922359316478675184342553333343300163568193749010867527082189412217781430311").unwrap()
          , Fp::from_str("91516472400306837063911995909475588197278444979245081960087094196120449075833").unwrap()
         ]
       , vec![ Fp::from_str("21304716730402869084944080869903443431235336418077153507261240151959530377653").unwrap()
          , Fp::from_str("106551237424345741137570659736231801772439680702621554106791455938098031620471").unwrap()
          , Fp::from_str("104392597313271110590927764888829150750277653499050463757708547416538850601163").unwrap()
         ]
       , vec![ Fp::from_str("16907937154215020261110468963982390213438461071031811101554056252102505124726").unwrap()
          , Fp::from_str("23183141532591565112222057191012766855134687114504142337903677590107533245206").unwrap()
          , Fp::from_str("96725517880771645283128624101279195709280644465575982072053504613644938879246").unwrap()
         ]
       , vec![ Fp::from_str("84556507395241990875812091718422997082915179448604219593521819129312718969906").unwrap()
          , Fp::from_str("100646525819453650494590571397259055384579251368754179569362740802641255820576").unwrap()
          , Fp::from_str("50316555026297423940834952362583934362215303629664094841692233643882339493043").unwrap()
         ]
       , vec![ Fp::from_str("77363534410783423412630139556441807611393685349073113946053979350631229049878").unwrap()
          , Fp::from_str("54905073434434959485893381841839373267383966385817882684657825178181863944371").unwrap()
          , Fp::from_str("110016011331508430102821620395154714608084938556260733745010992614542669817451").unwrap()
         ]
       , vec![ Fp::from_str("52040139270046094723964229965823921970388683619580004402190656733318120479093").unwrap()
          , Fp::from_str("495546618036723566920914648951352373868059898268055487677897567226892784967").unwrap()
          , Fp::from_str("2528292188392170914010448139211586215817069915670005292953294092269979070980").unwrap()
         ]
       , vec![ Fp::from_str("36842840134449713950999812540127591123318806680559982063089906871196226758113").unwrap()
          , Fp::from_str("112314504940338253416202605695368724580971154020421327790335219348068041886245").unwrap()
          , Fp::from_str("51653712314537383078368021242008468828072907802445786549975419682333073143987").unwrap()
         ]
       , vec![ Fp::from_str("27179054135131403873076215577181710354069071017096145081169516607932870071868").unwrap()
          , Fp::from_str("93264325401956094073193527739715293258814405715822269809955952297346626219055").unwrap()
          , Fp::from_str("75336695567377817226085396912086909560962335091652231383627608374094112503635").unwrap()
         ]
       , vec![ Fp::from_str("42536477740858058164730818130587261149155820207748153094480456895727052896150").unwrap()
          , Fp::from_str("45297707210835305388426482743535401273114010430724989418303851665124351001731").unwrap()
          , Fp::from_str("28263543670875633354854018109712021307749750769690268127459707194207091046997").unwrap()
         ]
       , vec![ Fp::from_str("40809484989590048522440442751358616303471639779690405026946053699354967624695").unwrap()
          , Fp::from_str("51589519265418587649124543325590658874910911006853535317847189422703251228717").unwrap()
          , Fp::from_str("73459936981642894525955700397592343967482441686326322443228255968694436816673").unwrap()
         ]
       , vec![ Fp::from_str("87298777232393189731949522229743081866971743270330772607820990832164835738703").unwrap()
          , Fp::from_str("23328534428894097247289332213412175849711532153957647506361455182140450133738").unwrap()
          , Fp::from_str("51807348624578081645565456865744011145427112815128832643950401419083788780028").unwrap()
         ]
       , vec![ Fp::from_str("62003629107726929116302469001779155132709624140360743951550189738290955064278").unwrap()
          , Fp::from_str("109311858027068383034683875948676795998030610067675200794951297783857157095297").unwrap()
          , Fp::from_str("2085588517087605436136379278738013214233743532079287631079316773925068862732").unwrap()
         ]
       , vec![ Fp::from_str("9513664655545306376987968929852776467090105742275395185801917554996684570014").unwrap()
          , Fp::from_str("91103467624252027317764670613760419385374004736848754250298970998535616755199").unwrap()
          , Fp::from_str("39500000352127197728032684892425352332461947514533659433380855624868454474623").unwrap()
         ]
       , vec![ Fp::from_str("75175260486328125629270378861920310368403601365269629778076078053196928460032").unwrap()
          , Fp::from_str("56923881233337629517433981230592855430598464522180216309153828833928801967999").unwrap()
          , Fp::from_str("20981004218820236011689230170078809973840534961691702543937445515733151438851").unwrap()
         ]
       , vec![ Fp::from_str("73175203586574092105626230272409823792532423094740797516874387144340145138310").unwrap()
          , Fp::from_str("45186992623753580336479418079070607289916086076906975839720879934817804495460").unwrap()
          , Fp::from_str("96084125187548549854900995260973117424750860440064269432639526863495781270780").unwrap()
         ]
       , vec![ Fp::from_str("53530507055579550362119832302266967544350117012822630711681736383163390079758").unwrap()
          , Fp::from_str("24484677147631687826970700541691541659768738376645174313438582486313045584324").unwrap()
          , Fp::from_str("99915577684197600584703320523786830947563355229812244982453188909016758004559").unwrap()
         ]
       , vec![ Fp::from_str("73101441225016284181831039876112223954723401962484828024235461623078642642543").unwrap()
          , Fp::from_str("57434882751817972247799186935032874577110609253567900895922769490031350316077").unwrap()
          , Fp::from_str("73837027842771758252813592393497967898989365991569964687267097531033696791279").unwrap()
         ]
       , vec![ Fp::from_str("8605586894544301092657394167906502995894014247978769840701086209902531650480").unwrap()
          , Fp::from_str("8900145888985471928279988821934068156350024482295663273746853580585203659117").unwrap()
          , Fp::from_str("76135096553134713603675854628257365311062159747768423095496501607463292188538").unwrap()
         ]
       , vec![ Fp::from_str("77171330825793179961995032914169307990870372845116475229799680315757656196917").unwrap()
          , Fp::from_str("17848856881287888035559207919717746181941756011012420474955535369227552058196").unwrap()
          , Fp::from_str("85285874363861776466393873037603415962379724376693393356387850868454172343232").unwrap()
         ]
       , vec![ Fp::from_str("34752820629818556525384193423224856177797869338806846583786365186093662702640").unwrap()
          , Fp::from_str("61923000676912108769617866333091286856690233713839015114991682235541391477568").unwrap()
          , Fp::from_str("105437294734850952102877811210027981435959945375626993201685688489494148805743").unwrap()
         ]
       , vec![ Fp::from_str("37290995592003925978648162243724313056459187397796644444696543576625771108605").unwrap()
          , Fp::from_str("95156804644588215637074780475000089186488581067063625121782605228712011438608").unwrap()
          , Fp::from_str("111838568780358037910894878973007194619694503969424695895292495245099084158661").unwrap()
         ]
       , vec![ Fp::from_str("114085830904535970531084512281741806703564152148485737755668141105183488387818").unwrap()
          , Fp::from_str("27151558900245092306095370161852910074651784795680581223133179808714387525774").unwrap()
          , Fp::from_str("17782273009863750298483603933610732253879825505411230932533407287574651036994").unwrap()
         ]
       , vec![ Fp::from_str("72422039981423868898452547270453235353957783762070405836433674391957844064693").unwrap()
          , Fp::from_str("23635533014670380888810554717349513178608213369182061967678315431422272271569").unwrap()
          , Fp::from_str("59402711345784829746976504521969665104448536964686633342173372133388407225657").unwrap()
         ]
       , vec![ Fp::from_str("92466806354851856571355165199186633833982438153589406912422876269386887264049").unwrap()
          , Fp::from_str("9877617390649361889067963484857474874019563445507538784053773745685676317984").unwrap()
          , Fp::from_str("74572672075215609948567780829046067891251792522874268554421916351892498078660").unwrap()
         ]
       , vec![ Fp::from_str("36552683919656073147232029802086505741533932059491323529262718897271096098319").unwrap()
          , Fp::from_str("28895802628889660292449057575076739706255701997961890168977786141673053679086").unwrap()
          , Fp::from_str("9907785227545441866241924986174555965766785257012652276622736289520175209842").unwrap()
         ]
       , vec![ Fp::from_str("29485332368911768475893015509537099136952860812699472744021496513325455451738").unwrap()
          , Fp::from_str("39797358509842904932758894391536601623578260107859540160156599261695054175926").unwrap()
          , Fp::from_str("107452259847197252302434271220963395311929879689430847107159618578878468880668").unwrap()
         ]
       , vec![ Fp::from_str("24664696127391052816688570667643612077905959307658722811431436096677076924072").unwrap()
          , Fp::from_str("52507998665481228083044018390203046881916651866666590845312076558622705190465").unwrap()
          , Fp::from_str("69935204723497468327083545368078327534124772251842862926136799697299751835029").unwrap()
         ]
       , vec![ Fp::from_str("372963191403207230700085823960930798511810380777302780932220121859190714585").unwrap()
          , Fp::from_str("111366606704792806959979488772421759791592911629496627207620326636856656861526").unwrap()
          , Fp::from_str("39677360977437767398760288273614298000827429534821360419179023551087917983124").unwrap()
         ]
       , vec![ Fp::from_str("64601494076430280535646633059501605929914790764963584476403188233843589027560").unwrap()
          , Fp::from_str("34156315098453482946438495274327282067376463494057110043754782161473776373661").unwrap()
          , Fp::from_str("73687667961196401152630755105477060056162632832680813161120412165243753726816").unwrap()
         ]
       , vec![ Fp::from_str("37808689229279738382348785246837013002280781984053433359148018860351753688153").unwrap()
          , Fp::from_str("26778210635417272626362658500217995247072424006327715268521424423461840656985").unwrap()
          , Fp::from_str("13012115310019421859484865413402512912208022868124085927375736053832542569552").unwrap()
         ]
       , vec![ Fp::from_str("33073055720188060063004545324174039863351833928493376423022587630016341635891").unwrap()
          , Fp::from_str("76584254259783109527449313057522305759653397147785066495263227868665161219779").unwrap()
          , Fp::from_str("38531270223194009551634352795926218399266465064491096474482575354468954922673").unwrap()
         ]
       , vec![ Fp::from_str("90100362566645034035707547984589905559141359276359522681964816611161474672115").unwrap()
          , Fp::from_str("93014643079204629081291124987233004565276697190519877698472422015954982964601").unwrap()
          , Fp::from_str("110916697765188052223435628742886773389228694903593626715469113528434066764534").unwrap()
         ]
       , vec![ Fp::from_str("114725280711584666069398481856753837363052938587178775403749719257369626174299").unwrap()
          , Fp::from_str("32967950615819700839673854548770413755655613096921050182183649674389310060672").unwrap()
          , Fp::from_str("106372438106855157117155417458821032302424106544646447353561892891697429919509").unwrap()
         ]
       , vec![ Fp::from_str("41996555998804572671679174634435850382099449308465335760130383677478780889948").unwrap()
          , Fp::from_str("105999190358126224751922865919841547624707481487885223948004296548330504340556").unwrap()
          , Fp::from_str("16636528128134911466622907961467317982179835733058354229921170933476186200761").unwrap()
         ]
       , vec![ Fp::from_str("43468498537738045222256870515315985487110433728199201952445121047095648527840").unwrap()
          , Fp::from_str("102272887089258604970815589009009162752025146641624347901234381428987386153285").unwrap()
          , Fp::from_str("797386830910520008361185815477523544664694040635544500916993469578452189812").unwrap()
         ]
       , vec![ Fp::from_str("96744926314199156321023598425708516126928808801578082649702497034531770517808").unwrap()
          , Fp::from_str("99066250188188051206024031106640566584616407903813704153928240609169764005797").unwrap()
          , Fp::from_str("101012485188852469291356197079506861083321680470016268483997462932491691773708").unwrap()
         ]
       , vec![ Fp::from_str("49614555470963378761214277525336169174318331863453657910575217035316990252780").unwrap()
          , Fp::from_str("94532874466332578813348267802784511494491757628599627802933242637211676358456").unwrap()
          , Fp::from_str("60376163781951477822973950330025689966951914888122503797194554488987660570913").unwrap()
         ]
       , vec![ Fp::from_str("99934768696780030317676638063039209891456597783633841250810260768328701786300").unwrap()
          , Fp::from_str("71861378641802240356627336242725340978135703736568776865558429280585792121426").unwrap()
          , Fp::from_str("84446994028646761779912629176051455275041688583492300440129402381138226185369").unwrap()
         ]
       , vec![ Fp::from_str("18317002472599225949038448120242542829985613745531554876060436499109578301758").unwrap()
          , Fp::from_str("23001721954642810524358122249469196372443463625490878969385130364780514025259").unwrap()
          , Fp::from_str("49037245410934285111914043557449391103989331168177809387278571893536129709378").unwrap()
         ]
       , vec![ Fp::from_str("65792050828386571136875573680568197513273253001530588336285451691348906024460").unwrap()
          , Fp::from_str("12956514709922286639520985225111137950302442378466870763868693820049405409474").unwrap()
          , Fp::from_str("38025781500219940187723501911749158551479941535921061459281014661810772473038").unwrap()
         ]
       , vec![ Fp::from_str("98610017124283789125637190759729078315864881693957982200427567103302362453196").unwrap()
          , Fp::from_str("42724178943656779288650125468921272883324869492775989038952508393082565227450").unwrap()
          , Fp::from_str("99514360136104778310983460863480701661882652836741972684579325226086664343913").unwrap()
         ]
       , vec![ Fp::from_str("111234788248236327826382691076985300771418365594838017963216100441270435887017").unwrap()
          , Fp::from_str("35290532009451633157074005614742321966918220860237810056920944192222599040501").unwrap()
          , Fp::from_str("72172784027306769601458922728374293130025170844011739475076742090414769211169").unwrap()
         ]
       , vec![ Fp::from_str("61384388429666858375759167984149961873566898632022783479711533101905095026411").unwrap()
          , Fp::from_str("8194273390415023152581060020119881338779571723515253104919314696738194355344").unwrap()
          , Fp::from_str("80659234466772556847544129237154202226081525079070598707001193889658539631883").unwrap()
         ]
       , vec![ Fp::from_str("62157670692624367146830864246105810519941474190553952682719433471854134465138").unwrap()
          , Fp::from_str("74851302400382275482762496406028988868219592716510355166137061257664688666219").unwrap()
          , Fp::from_str("16881796086836744646703159464114164393240695449455598565494759189917589453976").unwrap()
         ]
       , vec![ Fp::from_str("42460477269659081546432152357644086326197211166562674408623141905226706277595").unwrap()
          , Fp::from_str("81063688725529281621596607500748519671655516257039992655745871437369181665242").unwrap()
          , Fp::from_str("51403113216244137057466948399908740878535656059933892843818689317660325080213").unwrap()
         ]
       , vec![ Fp::from_str("49001998791770520786679099472805193463531142479298200795569326894791589887035").unwrap()
          , Fp::from_str("42684462014557597494725933597359625461226399711783671410492942446635214257509").unwrap()
          , Fp::from_str("106420886277092880545929306533089566775810130555230702838917980421765786292693").unwrap()
         ]
       , vec![ Fp::from_str("110523958037212353696746219917157237679515245560578307171595792811566554384451").unwrap()
          , Fp::from_str("56399709802930804752950401483879725014794413557467977624037632281590440364765").unwrap()
          , Fp::from_str("100108862073771478435824578087801736413858177140360408436521717282600830155374").unwrap()
         ]
       , vec![ Fp::from_str("59041409790855290045250456089216312297230856546920761548978870779493926213674").unwrap()
          , Fp::from_str("13735945315945382005247895569035266667172550063549145646185577935658666385507").unwrap()
          , Fp::from_str("16846296242516834547231537358954027537902709068158411294345086281698311539718").unwrap()
         ]
       , vec![ Fp::from_str("114970774262524353875592617323889610576992844847433725376114488262076142213525").unwrap()
          , Fp::from_str("17896661983150937411004047429485556264820315976705642986609974816436222162633").unwrap()
          , Fp::from_str("115573362005053049429141251153085446935774781295666612354309246218946442750706").unwrap()
         ]
       , vec![ Fp::from_str("85575265064375003235737272215445285540001719469558026661845214249857169530994").unwrap()
          , Fp::from_str("87501751332871186792668480006319719164949448258731670359536302677279100637346").unwrap()
          , Fp::from_str("105775909055063540416087237151517389942637625317333843436738223226977225420379").unwrap()
         ]
       , vec![ Fp::from_str("110886009455283422981396854898481256559291311408679418842391298005253375700608").unwrap()
          , Fp::from_str("95342257228100720685556647789433478371609336135456255803583405713563597933074").unwrap()
          , Fp::from_str("2733591517253510124338232417535938539627593736745105875672348998709544742241").unwrap()
         ]
       , vec![ Fp::from_str("32685479117496505057951010536248024091461630564950845696581129158987138920098").unwrap()
          , Fp::from_str("96139836598015371513111133071481139035733083963976340622322043979088723982681").unwrap()
          , Fp::from_str("16990606351055720221300633612533434675038905235719867684891402550154692840579").unwrap()
         ]
       , vec![ Fp::from_str("13886834869596827027283068322204563244577723967928602405785473534631482228259").unwrap()
          , Fp::from_str("81034769645830807786559566591578132114590768431721148809270219480247445931316").unwrap()
          , Fp::from_str("26780635035984131258327079447673207266040002451512601352288859614294714150612").unwrap()
         ]
       , vec![ Fp::from_str("72820784976920576285217524394309841044157743334874310886804718206719618858662").unwrap()
          , Fp::from_str("84276722913141806246805569560426345961854221390421557310593118606084442633714").unwrap()
          , Fp::from_str("42573817497593319926701003355299929070203785007821783512454795971915573843634").unwrap()
         ]
       , vec![ Fp::from_str("41660041627266397279909712983288860313881442148611073178272756605107913521726").unwrap()
          , Fp::from_str("7198246770791404776745997973411401046335399072925979346193035999274650139809").unwrap()
          , Fp::from_str("91576025129588718283317000330880100309465430116820675850311277329602716166005").unwrap()
         ]
       , vec![ Fp::from_str("30488483928716649313242898509172476460161184318124511942476380904233730958564").unwrap()
          , Fp::from_str("35346040517569327255933130090945133067049088493975862969006023114275649329148").unwrap()
          , Fp::from_str("59803015801166721680239913449555176591725421041660016242103441987856441941533").unwrap()
         ]
       , vec![ Fp::from_str("17395049232451382970906883167116397657891664802601579276725674512534883408665").unwrap()
          , Fp::from_str("96892830538146451450007413348096295684782382701592949711753407054447667361829").unwrap()
          , Fp::from_str("46725583995795907014628330473921842919957418641665482351238505922983315675600").unwrap()
         ]
       , vec![ Fp::from_str("20556719902345568138970735755829852608784985449829745172855204153387982836579").unwrap()
          , Fp::from_str("17130405757403641097651484965062131526367059595476924144885570325828777794585").unwrap()
          , Fp::from_str("99651763337265056372826178960800950053231370129318394703153246147873057668256").unwrap()
         ]
       , vec![ Fp::from_str("17814517977679061356584950826520510701145481336316888282105225134451035883368").unwrap()
          , Fp::from_str("62116749577126511600138536864540326578096290025961229483071769130930103978622").unwrap()
          , Fp::from_str("68057799973217998063838402481530957249181669394905338807621317159743376777292").unwrap()
         ]
       , vec![ Fp::from_str("26100793478962260035181580648528031417033872324944615961986573818448125345450").unwrap()
          , Fp::from_str("26507891451149179196332605230084404371370204632884553105363087566061809624465").unwrap()
          , Fp::from_str("55607174697006979796477169324630939518573410736589826596210132996613779221405").unwrap()
         ]
       , vec![ Fp::from_str("75098549092668095590746032937529532494350222003700838962461867948806312867882").unwrap()
          , Fp::from_str("62901674712278062473767645982006145910793625009149846534629441949336033280610").unwrap()
          , Fp::from_str("5918385816682866756860679567405784562483373873565987668410277610868983146285").unwrap()
         ]
       , vec![ Fp::from_str("99232460916208710346946062875203578399818909925477280432427620267031292402265").unwrap()
          , Fp::from_str("115165948144292852122635634954139515297086369356811820254801384608988902457684").unwrap()
          , Fp::from_str("39462036389170488019054739441325823641943062254145671230029238830857274014332").unwrap()
         ]
       , vec![ Fp::from_str("109723826013507458840008311671051963282645213933956581735587227292458581212170").unwrap()
          , Fp::from_str("88295699560808238817850908733435797366622278897489038929647807463406234520052").unwrap()
          , Fp::from_str("20306380368695786945008272690438693745796297843799468268993907772341096948885").unwrap()
         ]
       , vec![ Fp::from_str("39988356032524455736714109463355738665746339590560108227920970859248126609155").unwrap()
          , Fp::from_str("47372836588594871116561451142702593094337405740661755511614125456886719840333").unwrap()
          , Fp::from_str("42727075822142544969304941778878121065758237932060280070908539158139079888683").unwrap()
         ]
       , vec![ Fp::from_str("115695172202592006925180721060969710051274632819849770440984261650964182295350").unwrap()
          , Fp::from_str("41198034356969673080518030958715740186484860381424802035929938277103521577731").unwrap()
          , Fp::from_str("115380378505550001583545282887589851179279565072664241489053445702744491234750").unwrap()
         ]
       , vec![ Fp::from_str("56074680442485705900559544809779144521596408129059458559681779642734476672579").unwrap()
          , Fp::from_str("54187206076706271120400668422875039791522899135281309004702779376007885441827").unwrap()
          , Fp::from_str("100760509008368123701304764217450368711018785408289674422092070202193042774995").unwrap()
         ]
       , vec![ Fp::from_str("2622643323130765702269424224791233611920515499144047449633123689743564676648").unwrap()
          , Fp::from_str("95487289650450707799373930173349121410704227885014701702619711048222666102791").unwrap()
          , Fp::from_str("94943953462630479470879050547964792684799160133612590505176816568790562776928").unwrap()
         ] ]
    }
}
