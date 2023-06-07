// @generated
/// Implement [`DataProvider<MetazonePeriodV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_time_zone_metazone_period_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_TIME_ZONE_METAZONE_PERIOD_V1: &'static <icu_timezone::provider::MetazonePeriodV1Marker as icu_provider::DataMarker>::Yokeable = &icu_timezone::provider::MetazonePeriodV1(unsafe {
                #[allow(unused_unsafe)]
                zerovec::ZeroMap2d::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"adalv\0\0\0aedxb\0\0\0afkbl\0\0\0aganu\0\0\0aiaxa\0\0\0altia\0\0\0amevn\0\0\0ancur\0\0\0aolad\0\0\0aqcas\0\0\0aqdav\0\0\0aqddu\0\0\0aqmaw\0\0\0aqmcm\0\0\0aqplm\0\0\0aqrot\0\0\0aqsyw\0\0\0aqtrl\0\0\0aqvos\0\0\0arbue\0\0\0arcor\0\0\0arctc\0\0\0arirj\0\0\0arjuj\0\0\0arluq\0\0\0armdz\0\0\0arrgl\0\0\0arsla\0\0\0artuc\0\0\0aruaq\0\0\0arush\0\0\0asppg\0\0\0atvie\0\0\0auadl\0\0\0aubhq\0\0\0aubne\0\0\0audrw\0\0\0aueuc\0\0\0auhba\0\0\0aukns\0\0\0auldc\0\0\0auldh\0\0\0aumel\0\0\0aumqi\0\0\0auper\0\0\0ausyd\0\0\0awaua\0\0\0azbak\0\0\0basjj\0\0\0bbbgi\0\0\0bddac\0\0\0bebru\0\0\0bfoua\0\0\0bgsof\0\0\0bhbah\0\0\0bibjm\0\0\0bjptn\0\0\0bmbda\0\0\0bnbwn\0\0\0bolpb\0\0\0bqkra\0\0\0braux\0\0\0brbel\0\0\0brbvb\0\0\0brcgb\0\0\0brcgr\0\0\0brern\0\0\0brfen\0\0\0brfor\0\0\0brmao\0\0\0brmcz\0\0\0brpvh\0\0\0brrbr\0\0\0brrec\0\0\0brsao\0\0\0brssa\0\0\0brstm\0\0\0bsnas\0\0\0btthi\0\0\0bwgbe\0\0\0bymsq\0\0\0bzbze\0\0\0cacfq\0\0\0caedm\0\0\0caffs\0\0\0cafne\0\0\0caglb\0\0\0cagoo\0\0\0cahal\0\0\0caiql\0\0\0camon\0\0\0canpg\0\0\0capnt\0\0\0careb\0\0\0careg\0\0\0casjf\0\0\0cathu\0\0\0cator\0\0\0cavan\0\0\0cawnp\0\0\0caybx\0\0\0caycb\0\0\0cayda\0\0\0caydq\0\0\0cayek\0\0\0cayev\0\0\0cayxy\0\0\0cayyn\0\0\0cayzf\0\0\0cayzs\0\0\0cccck\0\0\0cdfbm\0\0\0cdfih\0\0\0cfbgf\0\0\0cgbzv\0\0\0chzrh\0\0\0ciabj\0\0\0ckrar\0\0\0clipc\0\0\0clpuq\0\0\0clscl\0\0\0cmdla\0\0\0cnsha\0\0\0cnurc\0\0\0cobog\0\0\0crsjo\0\0\0cst6cdt\0cuhav\0\0\0cvrai\0\0\0cxxch\0\0\0cyfmg\0\0\0cynic\0\0\0czprg\0\0\0deber\0\0\0debsngn\0djjib\0\0\0dkcph\0\0\0dmdom\0\0\0dosdq\0\0\0dzalg\0\0\0ecgps\0\0\0ecgye\0\0\0eetll\0\0\0egcai\0\0\0eheai\0\0\0erasm\0\0\0esceu\0\0\0eslpa\0\0\0esmad\0\0\0est5edt\0etadd\0\0\0fihel\0\0\0fimhq\0\0\0fjsuv\0\0\0fkpsy\0\0\0fmksa\0\0\0fmpni\0\0\0fmtkk\0\0\0fotho\0\0\0frpar\0\0\0galbv\0\0\0gazastrpgblon\0\0\0gdgnd\0\0\0getbs\0\0\0gfcay\0\0\0gggci\0\0\0ghacc\0\0\0gigib\0\0\0gldkshvnglgoh\0\0\0globy\0\0\0glthu\0\0\0gmbjl\0\0\0gmt\0\0\0\0\0gncky\0\0\0gpbbr\0\0\0gpmsb\0\0\0gpsbh\0\0\0gqssg\0\0\0grath\0\0\0gsgrv\0\0\0gtgua\0\0\0gugum\0\0\0gwoxb\0\0\0gygeo\0\0\0hebron\0\0hkhkg\0\0\0hntgu\0\0\0hrzag\0\0\0htpap\0\0\0hubud\0\0\0iddjj\0\0\0idjkt\0\0\0idmak\0\0\0idpnk\0\0\0iedub\0\0\0imdgs\0\0\0inccu\0\0\0iodga\0\0\0iqbgw\0\0\0irthr\0\0\0isrey\0\0\0itrom\0\0\0jeruslm\0jesth\0\0\0jmkin\0\0\0joamm\0\0\0jptyo\0\0\0kenbo\0\0\0kgfru\0\0\0khpnh\0\0\0kicxi\0\0\0kipho\0\0\0kitrw\0\0\0kmyva\0\0\0knbas\0\0\0kpfnj\0\0\0krsel\0\0\0kwkwi\0\0\0kygec\0\0\0kzaau\0\0\0kzakx\0\0\0kzala\0\0\0kzguw\0\0\0kzksn\0\0\0kzkzo\0\0\0kzura\0\0\0lavte\0\0\0lbbey\0\0\0lccas\0\0\0livdz\0\0\0lkcmb\0\0\0lrmlw\0\0\0lsmsu\0\0\0ltvno\0\0\0lulux\0\0\0lvrix\0\0\0lytip\0\0\0macas\0\0\0mcmon\0\0\0mdkiv\0\0\0metgd\0\0\0mgtnr\0\0\0mhkwa\0\0\0mhmaj\0\0\0mkskp\0\0\0mlbko\0\0\0mmrgn\0\0\0mncoq\0\0\0mnhvd\0\0\0mnuln\0\0\0momfm\0\0\0mpspn\0\0\0mqfdf\0\0\0mrnkc\0\0\0msmni\0\0\0mst7mdt\0mtmla\0\0\0muplu\0\0\0mvmle\0\0\0mwblz\0\0\0mxchi\0\0\0mxcjs\0\0\0mxcun\0\0\0mxhmo\0\0\0mxmam\0\0\0mxmex\0\0\0mxmid\0\0\0mxmty\0\0\0mxmzt\0\0\0mxoji\0\0\0mxpvr\0\0\0mxstis\0\0mxtij\0\0\0mykch\0\0\0mykul\0\0\0mzmpm\0\0\0nawdh\0\0\0ncnou\0\0\0nenim\0\0\0nfnlk\0\0\0nglos\0\0\0nimga\0\0\0nlams\0\0\0noosl\0\0\0npktm\0\0\0nrinu\0\0\0nuiue\0\0\0nzakl\0\0\0nzcht\0\0\0ommct\0\0\0papty\0\0\0pelim\0\0\0pfgmr\0\0\0pfnhv\0\0\0pfppt\0\0\0pgpom\0\0\0pgraw\0\0\0phmnl\0\0\0pkkhi\0\0\0plwaw\0\0\0pmmqc\0\0\0pnpcn\0\0\0prsju\0\0\0pst8pdt\0ptfnc\0\0\0ptlis\0\0\0ptpdl\0\0\0pwror\0\0\0pyasu\0\0\0qadoh\0\0\0rereu\0\0\0robuh\0\0\0rsbeg\0\0\0ruasf\0\0\0ruchita\0rudyr\0\0\0rugdx\0\0\0ruikt\0\0\0rukgd\0\0\0rukhndg\0rukra\0\0\0rukuf\0\0\0rumow\0\0\0runoz\0\0\0ruoms\0\0\0ruovb\0\0\0rupkc\0\0\0rurtw\0\0\0rusred\0\0ruuly\0\0\0ruunera\0ruuus\0\0\0ruvog\0\0\0ruvvo\0\0\0ruyek\0\0\0ruyks\0\0\0rwkgl\0\0\0saruh\0\0\0sbhir\0\0\0scmaw\0\0\0sdkrt\0\0\0sesto\0\0\0sgsin\0\0\0shshn\0\0\0silju\0\0\0sjlyr\0\0\0skbts\0\0\0slfna\0\0\0smsai\0\0\0sndkr\0\0\0somgq\0\0\0srpbm\0\0\0ssjub\0\0\0sttms\0\0\0svsal\0\0\0sxphi\0\0\0sydam\0\0\0szqmn\0\0\0tcgdt\0\0\0tdndj\0\0\0tfpfr\0\0\0tglfw\0\0\0thbkk\0\0\0tjdyu\0\0\0tkfko\0\0\0tldil\0\0\0tmasb\0\0\0tntun\0\0\0totbu\0\0\0trist\0\0\0ttpos\0\0\0tvfun\0\0\0twtpe\0\0\0tzdar\0\0\0uaiev\0\0\0uaozh\0\0\0uasip\0\0\0uauzh\0\0\0ugkla\0\0\0umawk\0\0\0umjon\0\0\0ummdy\0\0\0usadk\0\0\0usaeg\0\0\0usanc\0\0\0usboi\0\0\0uschi\0\0\0usden\0\0\0usdet\0\0\0ushnl\0\0\0usind\0\0\0usinvev\0usjnu\0\0\0usknx\0\0\0uslax\0\0\0uslui\0\0\0usmnm\0\0\0usmoc\0\0\0usmtm\0\0\0usndcnt\0usndnsl\0usnyc\0\0\0usoea\0\0\0usome\0\0\0usphx\0\0\0ussit\0\0\0ustel\0\0\0uswlz\0\0\0uswsq\0\0\0usxul\0\0\0usyak\0\0\0uymvd\0\0\0uzskd\0\0\0uztas\0\0\0vavat\0\0\0vcsvd\0\0\0veccs\0\0\0vgtov\0\0\0vistt\0\0\0vnsgn\0\0\0vuvli\0\0\0wfmau\0\0\0wsapw\0\0\0yeade\0\0\0ytmam\0\0\0zajnb\0\0\0zmlun\0\0\0zwhre\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x08\0\0\0\t\0\0\0\n\0\0\0\x16\0\0\0\x17\0\0\0\x18\0\0\0\x19\0\0\0\x1A\0\0\0\x1C\0\0\0\x1D\0\0\0\x1E\0\0\0\x1F\0\0\0 \0\0\0!\0\0\0#\0\0\0'\0\0\0,\0\0\0.\0\0\x007\0\0\0:\0\0\0=\0\0\0?\0\0\0C\0\0\0H\0\0\0J\0\0\0L\0\0\0M\0\0\0N\0\0\0O\0\0\0P\0\0\0Q\0\0\0R\0\0\0S\0\0\0T\0\0\0U\0\0\0W\0\0\0X\0\0\0Y\0\0\0Z\0\0\0[\0\0\0\\\0\0\0^\0\0\0_\0\0\0`\0\0\0b\0\0\0c\0\0\0d\0\0\0e\0\0\0g\0\0\0h\0\0\0i\0\0\0j\0\0\0k\0\0\0l\0\0\0m\0\0\0n\0\0\0o\0\0\0p\0\0\0q\0\0\0r\0\0\0u\0\0\0v\0\0\0w\0\0\0x\0\0\0y\0\0\0z\0\0\0}\0\0\0~\0\0\0\x7F\0\0\0\x80\0\0\0\x82\0\0\0\x83\0\0\0\x85\0\0\0\x86\0\0\0\x8A\0\0\0\x8B\0\0\0\x8C\0\0\0\x8D\0\0\0\x8E\0\0\0\x90\0\0\0\x91\0\0\0\x94\0\0\0\x95\0\0\0\x98\0\0\0\x99\0\0\0\x9A\0\0\0\x9D\0\0\0\xA2\0\0\0\xA3\0\0\0\xA4\0\0\0\xA5\0\0\0\xA6\0\0\0\xA7\0\0\0\xA8\0\0\0\xA9\0\0\0\xAE\0\0\0\xB0\0\0\0\xB2\0\0\0\xB5\0\0\0\xB7\0\0\0\xB9\0\0\0\xBB\0\0\0\xBC\0\0\0\xBD\0\0\0\xBE\0\0\0\xBF\0\0\0\xC0\0\0\0\xC1\0\0\0\xC2\0\0\0\xC3\0\0\0\xC4\0\0\0\xC5\0\0\0\xC6\0\0\0\xC7\0\0\0\xC8\0\0\0\xC9\0\0\0\xCA\0\0\0\xCB\0\0\0\xCC\0\0\0\xCD\0\0\0\xCE\0\0\0\xCF\0\0\0\xD0\0\0\0\xD1\0\0\0\xD2\0\0\0\xD3\0\0\0\xD4\0\0\0\xD5\0\0\0\xD6\0\0\0\xD7\0\0\0\xD8\0\0\0\xD9\0\0\0\xDD\0\0\0\xE1\0\0\0\xE3\0\0\0\xE4\0\0\0\xE6\0\0\0\xE7\0\0\0\xE9\0\0\0\xEA\0\0\0\xEC\0\0\0\xED\0\0\0\xEE\0\0\0\xEF\0\0\0\xF0\0\0\0\xF1\0\0\0\xF2\0\0\0\xF3\0\0\0\xF4\0\0\0\xF5\0\0\0\xF6\0\0\0\xF7\0\0\0\xF8\0\0\0\xF9\0\0\0\xFA\0\0\0\xFC\0\0\0\xFE\0\0\0\xFF\0\0\0\x01\x01\0\0\x02\x01\0\0\x04\x01\0\0\x05\x01\0\0\x06\x01\0\0\x08\x01\0\0\t\x01\0\0\x0B\x01\0\0\x0C\x01\0\0\r\x01\0\0\x0E\x01\0\0\x0F\x01\0\0\x10\x01\0\0\x11\x01\0\0\x12\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0\x18\x01\0\0\x1A\x01\0\0\x1B\x01\0\0\x1D\x01\0\0\x1E\x01\0\0\x1F\x01\0\0 \x01\0\0!\x01\0\0\"\x01\0\0#\x01\0\0$\x01\0\0%\x01\0\0'\x01\0\0)\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0/\x01\0\x000\x01\0\x001\x01\0\x002\x01\0\x004\x01\0\x005\x01\0\x006\x01\0\x007\x01\0\08\x01\0\0:\x01\0\0;\x01\0\0<\x01\0\0=\x01\0\0>\x01\0\0?\x01\0\0@\x01\0\0C\x01\0\0D\x01\0\0E\x01\0\0F\x01\0\0I\x01\0\0L\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0T\x01\0\0W\x01\0\0X\x01\0\0Y\x01\0\0Z\x01\0\0[\x01\0\0^\x01\0\0`\x01\0\0a\x01\0\0e\x01\0\0f\x01\0\0h\x01\0\0o\x01\0\0r\x01\0\0s\x01\0\0u\x01\0\0v\x01\0\0w\x01\0\0y\x01\0\0z\x01\0\0{\x01\0\0|\x01\0\0}\x01\0\0\x80\x01\0\0\x81\x01\0\0\x82\x01\0\0\x84\x01\0\0\x86\x01\0\0\x87\x01\0\0\x88\x01\0\0\x89\x01\0\0\x8A\x01\0\0\x8B\x01\0\0\x8C\x01\0\0\x8D\x01\0\0\x8E\x01\0\0\x91\x01\0\0\x95\x01\0\0\x99\x01\0\0\x9B\x01\0\0\x9C\x01\0\0\x9D\x01\0\0\xA0\x01\0\0\xA1\x01\0\0\xA3\x01\0\0\xA6\x01\0\0\xA9\x01\0\0\xAA\x01\0\0\xAB\x01\0\0\xAD\x01\0\0\xAF\x01\0\0\xB0\x01\0\0\xB4\x01\0\0\xB5\x01\0\0\xB6\x01\0\0\xB7\x01\0\0\xB8\x01\0\0\xBF\x01\0\0\xC0\x01\0\0\xC1\x01\0\0\xC2\x01\0\0\xC3\x01\0\0\xC4\x01\0\0\xC5\x01\0\0\xC6\x01\0\0\xC7\x01\0\0\xC8\x01\0\0\xC9\x01\0\0\xCA\x01\0\0\xCB\x01\0\0\xCC\x01\0\0\xCD\x01\0\0\xCE\x01\0\0\xCF\x01\0\0\xD1\x01\0\0\xD2\x01\0\0\xD4\x01\0\0\xD5\x01\0\0\xD6\x01\0\0\xD7\x01\0\0\xD8\x01\0\0\xDC\x01\0\0\xDF\x01\0\0\xE0\x01\0\0\xE1\x01\0\0\xE3\x01\0\0\xE4\x01\0\0\xE5\x01\0\0\xE6\x01\0\0\xE7\x01\0\0\xEA\x01\0\0\xED\x01\0\0\xEE\x01\0\0\xEF\x01\0\0\xF3\x01\0\0\xF6\x01\0\0\xF7\x01\0\0\xFB\x01\0\0\xFE\x01\0\0\x01\x02\0\0\x02\x02\0\0\x03\x02\0\0\x04\x02\0\0\x05\x02\0\0\x06\x02\0\0\x07\x02\0\0\n\x02\0\0\x0B\x02\0\0\x0C\x02\0\0\r\x02\0\0\x0F\x02\0\0\x10\x02\0\0\x11\x02\0\0\x12\x02\0\0\x13\x02\0\0\x14\x02\0\0\x17\x02\0\0\x18\x02\0\0\x19\x02\0\0\x1A\x02\0\0\x1B\x02\0\0\x1C\x02\0\0\x1D\x02\0\0\x1E\x02\0\0\x1F\x02\0\0 \x02\0\0!\x02\0\0#\x02\0\0&\x02\0\0)\x02\0\0*\x02\0\0+\x02\0\0,\x02\0\0-\x02\0\x000\x02\0\x001\x02\0\x002\x02\0\x003\x02\0\x004\x02\0\x006\x02\0\x007\x02\0\0:\x02\0\0<\x02\0\0=\x02\0\0>\x02\0\0B\x02\0\0C\x02\0\0D\x02\0\0E\x02\0\0F\x02\0\0H\x02\0\0J\x02\0\0O\x02\0\0Q\x02\0\0R\x02\0\0S\x02\0\0U\x02\0\0W\x02\0\0Y\x02\0\0\\\x02\0\0^\x02\0\0_\x02\0\0`\x02\0\0a\x02\0\0b\x02\0\0d\x02\0\0e\x02\0\0f\x02\0\0i\x02\0\0l\x02\0\0m\x02\0\0p\x02\0\0r\x02\0\0t\x02\0\0x\x02\0\0z\x02\0\0|\x02\0\0}\x02\0\0\x80\x02\0\0\x82\x02\0\0\x83\x02\0\0\x85\x02\0\0\x87\x02\0\0\x8A\x02\0\0\x8E\x02\0\0\x90\x02\0\0\x91\x02\0\0\x92\x02\0\0\x96\x02\0\0\x98\x02\0\0\x99\x02\0\0\x9A\x02\0\0\x9B\x02\0\0\x9C\x02\0\0\x9D\x02\0\0\x9E\x02\0\0\x9F\x02\0\0\xA0\x02\0\0\xA1\x02\0\0\xA2\x02\0\0\xA3\x02\0\0\xA4\x02\0\0\xA5\x02\0\0\xA6\x02\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0pY\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\xF8]?\x01\x84eB\x01x\xA0O\x01\\2R\x01 \xA3w\x01\xFC\xBB\x82\x01\xF0Y\x87\x01\xA0\xE2\x8A\x01\xF4M\x8F\x01\xC0\xBA\x92\x01\x01X\x97\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\xF2b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\xF3\xAE\0\0\0\0\x000\xF3\xAE\0\xF42\x14\x01\x10\x9E\x14\x01\0\0\0\0\x18\xD4\xA9\0pM\xAB\0\xF42\x14\x01\x10\x9E\x14\x01\0\0\0\0p\xA4\xAE\0\0\0\0\0\x18\x18\xA2\0\x10\xDA\xAB\0\x94\xC7\xEE\0\x94\x1E\xF2\0T-\x14\x01\xF0b\x15\x01Xf1\x01\xF08?\x01\0\0\0\0\xB0\xF2\xB6\0P\xC5\x16\x01\0\0\0\0\xF42\x14\x01\x10\x9E\x14\x01\0\0\0\x000\xF3\xAE\0\0\0\0\x000\xF3\xAE\0\xF42\x14\x01\xB0v\x14\x01\0\0\0\0\x18\xD4\xA9\0pM\xAB\0T-\x14\x01\xF0b\x15\x01\0\0\0\0\x10\x9E\x14\x01\0\0\0\0\x10\xF9n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC8\x92Y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0p\xD2\xAD\0\0\0\0\0\0\0\0\0\0\0\0\08\xDC\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0P`\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xEC\xCE4\x01\xD0\xFB_\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xEC\xCE4\x01\xD0\xFB_\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xB0\xCE4\x01\0\0\0\0\0\0\0\0\xF6k\x8E\0\0\0\0\0\0\0\0\0$|\xAA\0\x80\xE8J\x01\xC8\xB0g\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18\x9Aj\x01\0\0\0\0\0\0\0\0\xD1~\x92\0\x99\x1B\x97\0\0\0\0\0\0\0\0\0\xC8e\xEF\0\x84e\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\xC8e\xEF\0\x84e\xF7\0\0\0\0\0\x84e\xF7\0\0\xC8\xFA\0\xE4\x89'\x01@v*\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0@f\xEF\0\x84e\xF7\0l\x8C\xF7\0<\xC8\xFA\0\x9C\xAE\x1E\0\x04\xF9\x97\x01\0\0\0\0\x9C]\x15\0\0\0\0\0\x84e\xF7\0\0\xC8\xFA\0\0\0\0\0x\xD3J\0\0\0\0\0\x04\xF9\x97\x01\0\0\0\0\\\xAF\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0,\xAD&\0He\xF7\0(*\xF8\0\0\0\0\0\x04\x9F>\0d\xC5N\0@\xECZ\0\0\0\0\0\xACi\x80\0\0\0\0\0\0\0\0\0\xC4U\x9A\0\0\0\0\0\0\0\0\0\x9Cm2\0\0\0\0\0\0\0\0\0\x80\xFEq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\xA6\xD0\0\0\0\0\0\xF8\xAD\x0E\0\0\0\0\0\0\0\0\0\x10\xAE\xAA\0\0\0\0\0\0\0\0\0\xF8\xAD\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\xB4\xA7\xD0\0\0\0\0\0\0\0\0\0\x183Z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE8\x96\xF8\0\0\0\0\0|\x1F(\0\0\0\0\0\0\0\0\0\x88\xA6\xD0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE0p\x90\0\0\0\0\0\xF8\xAD\x0E\0\0\0\0\0\xF8\xAD\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xF8\xAD\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x10\xD8\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xA4\x19n\x01\xE4\xF0\x83\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xB41\xB0\0\xC8\x88\x17\x01\0\0\0\0\xB41\xB0\0\x8C\x88\x17\x01\0\0\0\0P\x88\x17\x01\xC8\x88\x17\x01\x8C\x88\x17\x01\0\0\0\0\xB41\xB0\0\x8C\x88\x17\x01X\xFF\x88\x01\0\0\0\0\xF01\xB0\0\xC8\x88\x17\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0V\xD5\xD3\0\xD62#\x01\0\0\0\0\r\xB3\x12\0\0\0\0\0\0\0\0\0\xC4U\x9A\0|\xA0\xE2\0\x9Cd\xEF\0\0\0\0\0\0\0\0\0\xC4U\x9A\0\0\0\0\0\xE8M`\0D6\xA3\0(\xA6\xD6\0\xC8\xC1\xDE\0\xC0\xF5W\x01\xE0\xA0_\x01\0\0\0\0\x80\xFEq\0Dh\x80\0\0\0\0\0\0\0\0\0\x0CA\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0p\xB3\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE0Kj\0\x84\xED2\x01\0\0\0\0\0\0\0\0\0\0\0\0\xC0{\xF0\0\0\0\0\0\xE8\x96\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xBC\xC9\xE2\0@\xF8\xA7\x01\0\0\0\0\xBC\xC9\xE2\0@\xF8\xA7\x01(\xA6\xA8\x01\0\0\0\0(\x1D`\0hf\xE5\0\xC0\xD4i\x01\0\0\0\0\xE0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0(\x1D`\0\xEC\xABg\0\0\0\0\0\0\0\0\0\xE0\x01\0\0\0\0\0\0\xBC\xC9\xE2\0@\xF8\xA7\x01\0\0\0\0\xE0\x01\0\0|\x12C\x01\0\0\0\0\0\0\0\0\0\0\0\0\x80L`\0\0\0\0\0\x9EL`\0\0\0\0\0\0\0\0\0\x88>\xA2\0\xA8X\xC2\0\xE8\xB4\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0h\xB9\x1A\0,#)\08\x8F\xB0\0\xECk\xB6\0\x08\x99\xB8\0\xEC\xB2\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0t\xDC\t\0\0\0\0\0\0\0\0\0\x10\xE8R\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\r6\0\xDC{\xB6\0|\xA1\xD2\0\0\0\0\0\x18|\xB6\0\x9C{\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0P`\x13\0\0\0\0\0\0\0\0\0\0\0\0\0h{\xB2\0\0\0\0\0\xC0\xA9g\x01\xF8\x0Bs\x01\0\0\0\0\xA8\xE6B\x01H\xA9g\x01\0\0\0\0\0\0\0\0\0\0\0\0\xC4U\x9A\0\x80\xE8J\x01d\xABg\x01\0\0\0\0$\xD9\x10\x01,\xA2N\x01\0\0\0\0\0\0\0\0\x88U\x9A\0$|\xAA\0 |\xAE\0\0\0\0\0$|\xAA\0 \xF2\xB0\0\0\0\0\0\xD4\xE7B\x01t\xAAg\x01\0\0\0\0\0\0\0\0\0\0\0\0h{\xB2\0\0\0\0\0 \xF2\xB0\0\0\0\0\0dAZ\0\xF0\xA1N\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xA8\xF1\xB0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\08\x12\xF1\0\xAC\xE1\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xF28/\0\0\0\0\08\x12\xF1\0,\xFC\x99\x01\0\0\0\0\xBC9\x81\x01\xDC>\x89\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0d\x99j\x01D\xBF\x82\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xEC\n\xAE\0\0\0\0\0\0\0\0\0$\xD62\0\xC0u\xF6\0\0\0\0\0\xB0\x18\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0l\"D\0\xCC\xC7z\0,\xA7v\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0H|\xA4\0\0\0\0\0H|\xA4\0\0\0\0\0\x84|\xA4\0\x0C?\xC3\0\xFC\xA0\xDA\0`\x0Ec\x01\0\0\0\0H|\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\xD4\xF8n\0\0\0\0\0\x10\xF9n\0\0\0\0\0\xF8\xA6o\0\0\0\0\0\xE47 \0\xA4\xAD&\0\0\0\0\0\xBC\xA6o\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xD4\xF8n\0\0\0\0\0\0\0\0\0\0\0\0\0\xB8\xD2V\0\xBC\xA6o\0\0\0\0\0D\x1B\xAF\0\xA4\xEC\"\x01\0\0\0\0\0\0\0\0\xE47 \0\xA4\xAD&\0\0\0\0\0d\xAE\x1A\0\0\0\0\0\x84e\xF7\0\0\0\0\0\xD8\xD4o\x01\xB8\xFA\x87\x01\xD8\xAB\x89\x01\0\0\0\0\0\x1B\xB7\0\0\0\0\0@d\x0F\x01\0\0\0\0\0\0\0\0\xA4\xEC\"\x01\xC4\xB0/\x01\0\0\0\0\xBC\xA6o\0\0\0\0\0\0\0\0\0\xBC\xA6o\0\0\0\0\0\xA4\xEC\"\x01\0\0\0\0\xA4\xEC\"\x01@v*\x01\0\0\0\0\x84\xD3>\0\xA4\xEC\"\x01\xC4\xB0/\x01\0\0\0\0\xE0\xD6G\x01\xBC\xA6o\0\0\0\0\0\0\0\0\0xG^\08Gb\08\xDD\xAD\0\0\0\0\08\xDD\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0@\xB2+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01euce\x01gulf\x01afgh\x01atla\x01atla\x01euce\0\0\0\0\0\x01arme\x01atla\x01afwe\x01auwe\x01case\x01auwe\x01case\x01auwe\x01case\x01auwe\x01case\x01auwe\x01case\x01auwe\x01case\x01davi\x01dumo\x01maws\x01neze\x01arge\x01chil\x01roth\x01syow\x01mgmt\x01vost\x01arge\x01arge\x01arge\x01arge\x01arge\x01arwe\x01arge\x01arge\x01arwe\x01arge\x01arwe\x01arge\x01arge\x01arge\x01arge\x01arwe\x01arge\x01arwe\x01arge\x01arwe\x01arge\x01arwe\x01arge\x01arge\x01arge\x01arge\x01arge\x01arwe\x01arge\x01arge\x01arge\x01arge\x01arge\x01arwe\x01arge\x01arge\x01arwe\x01arge\x01arwe\x01arge\x01arge\x01arge\0\0\0\0\0\x01samo\x01euce\x01auce\x01auce\x01auea\x01auce\x01aucw\x01auea\x01auea\x01auea\x01auea\x01loho\x01auea\x01auea\x01auwe\x01auea\x01atla\0\0\0\0\0\x01azer\x01euce\x01atla\0\0\0\0\0\x01bang\x01euce\x01mgmt\x01euea\x01gulf\x01arab\x01afce\x01afwe\x01atla\x01brun\x01boli\x01atla\x01bras\x01bras\x01amaz\x01amaz\x01amaz\x01acre\x01amaz\x01acre\x01noro\x01bras\x01amaz\x01bras\x01amaz\x01acre\x01amaz\x01acre\x01bras\x01bras\x01bras\x01amaz\x01bras\x01amea\x01indi\x01bhut\x01afce\x01mosc\x01euea\x01eufe\x01mosc\x01amce\x01ammo\x01ammo\x01amce\x01ampa\x01ammo\x01atla\x01atla\0\0\0\0\0\x01atla\x01atla\x01amea\x01amce\x01amea\x01atla\x01amea\x01amea\x01amce\x01amea\x01amce\x01amea\x01amce\x01amea\x01amce\x01amce\x01newf\x01amea\x01amea\x01ampa\x01amce\x01atla\x01ammo\x01amce\x01amea\x01amce\x01ammo\x01ampa\x01yuko\x01ampa\x01ammo\x01amce\x01amea\x01amce\x01ampa\x01ammo\x01ampa\x01yuko\x01ammo\x01amce\x01ammo\x01amea\x01coco\x01afce\x01afwe\x01afwe\x01afwe\x01euce\x01mgmt\x01cook\x01east\x01chil\x01chil\x01afwe\x01chin\0\0\0\0\0\x01colo\x01amce\x01amce\x01cuba\x01cave\x01chri\x01euea\x01euea\x01euce\x01euce\x01euce\x01afea\x01euce\x01atla\0\0\0\0\0\x01atla\x01amea\x01atla\x01euwe\x01euce\x01euwe\x01euce\x01ecua\x01gala\x01ecua\x01mosc\x01euea\x01euea\0\0\0\0\0\x01euwe\x01afea\x01euwe\x01euce\x01euwe\x01euce\x01amea\x01afea\x01euea\x01euea\x01fiji\x01falk\x01kosr\x01pona\x01truk\x01euwe\x01euce\x01afwe\x01isra\x01euea\0\0\0\0\0\x01mgmt\x01atla\0\0\0\0\0\x01geor\x01frgu\0\0\0\0\0\x01mgmt\x01mgmt\x01euce\x01grwe\x01mgmt\x01grwe\0\0\0\0\0\x01grea\x01atla\x01mgmt\x01mgmt\x01mgmt\x01atla\x01atla\x01atla\x01afwe\x01euea\x01soge\x01amce\x01guam\x01cham\0\0\0\0\0\x01mgmt\x01guya\x01isra\x01euea\x01hoko\x01amce\x01euce\x01amea\x01euce\x01inea\x01inwe\x01ince\x01ince\x01inwe\0\0\0\0\0\x01mgmt\0\0\0\0\0\x01mgmt\x01indi\x01inoc\x01arab\x01iran\x01mgmt\x01euce\x01isra\0\0\0\0\0\x01mgmt\x01amea\x01euea\x01japa\x01afea\0\0\0\0\0\x01kyrg\x01indo\x01liis\x01phis\x01giis\x01afea\x01atla\x01kore\x01pyon\x01kore\x01kore\x01arab\x01amea\0\0\0\0\0\x01aqta\x01kawe\0\0\0\0\0\x01aqto\x01kawe\x01alam\x01kaea\x01kawe\x01kaea\0\0\0\0\0\x01qyzy\x01kaea\x01kawe\0\0\0\0\0\0\0\0\0\0\x01kawe\x01indo\x01euea\x01atla\x01euce\x01indi\x01lank\x01indi\0\0\0\0\0\x01mgmt\x01afso\x01mosc\x01euea\x01euce\x01euea\x01euce\x01mosc\x01euea\x01euea\x01euce\x01euea\x01euce\x01euea\x01euce\x01euea\x01euwe\x01euce\x01euwe\x01euce\x01mosc\x01euea\x01euce\x01afea\0\0\0\0\0\x01mais\x01mais\x01euce\x01mgmt\x01myan\x01mong\x01choi\x01mong\x01hovd\x01mong\x01maca\x01chin\x01noma\x01cham\x01atla\x01mgmt\x01atla\x01ammo\x01euce\x01maur\x01mald\x01afce\x01amce\x01mepa\x01amce\x01amce\x01mepa\x01amce\x01ammo\x01amce\x01amea\x01amce\x01amea\x01ampa\x01mepa\x01amce\x01amce\x01amce\x01amea\x01amce\x01amce\x01ampa\x01mepa\x01amce\x01ammo\x01amce\x01ampa\x01ammo\x01amce\x01meno\x01ampa\0\0\0\0\0\x01mala\0\0\0\0\0\x01mala\x01afce\x01afso\x01afce\x01afwe\x01afce\x01neca\x01afwe\x01norf\x01afwe\x01amce\x01amea\x01amce\x01amea\x01amce\x01amea\x01amce\x01euce\x01euce\x01nepa\x01naur\x01niue\x01neze\x01chat\x01gulf\x01amea\x01peru\x01gamb\x01marq\x01tahi\x01pang\x01pang\x01phil\0\0\0\0\0\x01paki\x01euce\x01atla\x01pimi\x01pitc\x01atla\x01ampa\x01euwe\x01euce\x01euwe\x01euce\x01euwe\x01azor\x01euwe\x01azor\x01pala\x01para\x01gulf\x01arab\x01reun\x01euea\x01euce\x01mosc\x01yaku\x01irku\x01yaku\x01anad\x01maga\x01anad\x01maga\x01irku\x01mosc\x01euea\x01eufe\x01euea\x01yaku\x01vlad\x01yaku\x01kras\0\0\0\0\0\x01mosc\x01euea\x01sama\x01mosc\x01euea\x01mosc\x01kras\x01novo\x01kras\x01omsk\x01novo\x01kamc\x01mosc\x01maga\x01mosc\x01yaku\x01maga\x01vlad\x01sakh\x01volg\x01vlad\0\0\0\0\0\x01yeka\x01yaku\x01afce\x01arab\x01solo\x01seyc\x01afce\x01afea\x01afce\x01euce\x01sing\x01mgmt\x01euce\x01euce\x01euce\x01mgmt\x01euce\x01mgmt\x01afea\0\0\0\0\0\x01suri\x01afce\x01afea\x01afce\x01mgmt\x01afwe\x01mgmt\x01amce\x01atla\x01euea\x01afso\x01amea\x01atla\x01amea\x01afwe\x01frso\x01mgmt\x01indo\0\0\0\0\0\x01taji\x01toke\x01eati\x01ince\x01eati\0\0\0\0\0\x01turk\x01euce\x01tong\x01euea\0\0\0\0\0\x01euea\0\0\0\0\0\x01atla\x01tuva\x01taip\x01afea\x01mosc\x01euea\x01mosc\x01euea\x01mosc\x01euea\x01mosc\x01euea\x01mosc\x01mosc\x01euea\x01afea\x01wake\0\0\0\0\0\x01haal\0\0\0\0\0\x01samo\0\0\0\0\0\x01haal\x01amea\x01amce\x01amea\0\0\0\0\0\x01alas\x01ammo\x01amce\x01ammo\x01amea\0\0\0\0\0\x01haal\x01amea\x01amea\x01ampa\x01ampa\x01alas\x01amce\x01amea\x01amce\x01ampa\x01amea\x01amce\x01amea\x01amea\x01amce\x01amce\x01amea\x01ampa\x01alas\x01ampa\x01alas\x01ammo\x01amce\x01ammo\x01amce\x01amea\x01amea\x01amce\x01amea\0\0\0\0\0\x01alas\x01ammo\x01ampa\x01alas\x01amea\x01amce\x01amea\x01amce\x01amea\x01amce\x01amea\x01amce\x01amea\x01ammo\x01amce\x01alas\x01urug\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01uzbe\0\0\0\0\0\x01uzbe\x01euce\x01atla\x01vene\x01atla\x01atla\x01indo\x01vanu\x01wall\x01apia\x01arab\x01afea\x01afso\x01afce\x01afce") })
            });
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_timezone::provider::MetazonePeriodV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_timezone::provider::MetazonePeriodV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    if locale.is_empty() {
                        Ok(Self::SINGLETON_TIME_ZONE_METAZONE_PERIOD_V1)
                    } else {
                        Err(icu_provider::DataErrorKind::ExtraneousLocale)
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_timezone::provider::MetazonePeriodV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
