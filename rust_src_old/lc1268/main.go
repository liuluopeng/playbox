package main

import "fmt"

type TireNode struct {
	Next  [26]*TireNode
	IsEnd bool
}

func (node *TireNode) Insert(word string) {

	for _, char := range word {
		index := char - 'a'
		if node.Next[index] == nil {
			node.Next[index] = &TireNode{}
		}
		node = node.Next[index]
	}

	node.IsEnd = true
}



func (node *TireNode) StartWith(word string) *TireNode { // 找到前缀的节点,处于中间的位置.
	curr := node
	for _, v := range word {
		index := v - 'a'
		if curr.Next[index] == nil {
			return nil
		}
		curr = curr.Next[index]
	}
	return curr
}

func (node *TireNode) IsWord(prev string) bool {

	curr := node
	for _, v := range prev {
		index := v - 'a'
		if curr.Next[index] == nil {
			return false
		}
		curr = curr.Next[index]
	}
	if curr.IsEnd == true {
		return true
	}
	return false
}

func (node TireNode) GetALL(prev string) []string {

	startNode := node.StartWith(prev)

	if startNode == nil {
		return []string{}
	}

	var dfs func(node *TireNode, sum string, index int)
	res := []string{}

	if node.IsWord(prev) {
		res = append(res, prev)
	}

	dfs = func(node *TireNode, sum string, index int) {
		for i, v := range node.Next {
			if v != nil {
				char := byte(i + 97)
				newSum := sum + string(char)
				if v.IsEnd && len(res) < 3 {
					res = append(res, prev+newSum)
				}
				dfs(node.Next[i], newSum, index+1)
			}
		}
	}

	dfs(startNode, "", 0)

	// fmt.Println(res)

	return res
}



func suggestedProducts(products []string, searchWord string) [][]string {
	node := &TireNode{}

	for _, word := range products {
		//
		node.Insert(word)
	}

	res := [][]string{}

	searchWord_byte := []byte(searchWord)

	for i, _ := range searchWord_byte {
		// 创建 m  mo  mou  mous ...
		// fmt.Println(i, st, string(searchWord_byte[:i+1]))

		se_word := string(searchWord_byte[:i+1])

		// res = append(res, node.Search(se_word))

		all := node.GetALL(se_word)

		// one_res := findInAll(all, se_word)

		res = append(res, all)
	}

	return res
}

func main() {
	ps := []string{"mobile", "mouse", "moneypot", "monitor", "mousepad"}
	sw := "mouse"

	// ps = []string{"tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnunqerptgas", "zmmirsxdhyxvmdybjzondyvrkzeafhvualsivfueweuusmsxbttdeofzeripaqv", "tyqcpfvorznmxxdzepfxabibcagilwjsqncxnpjqsxjzqqqbae", "tyqcpfvacyrjvmadrmntxotgvgivdvcuwygpjfwcuppunolukrum", "tyqcpfvrqwrcpusmfyhxaiasfbb", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqyalwiaj", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjbdchidnpt", "lfjkcljnd", "zibrwfpwecubjlsjbkrhnvolnnzrqhdynloplzagwnuhpxhbvpxnqaifnln", "ybswoottdgryxtupichpvqjmcoytrwnfgzrrnaejojvpzmttlzw", "tyqcplghosxjviooiyddhvzzrhuuwkiosmgafxyajcdyqlmthqkoylxhtxdruw", "okoscfpxcndzgbtsozdofgnomtglmoaewgzzjvrxezoq", "cxkwvaytkxgafeltbanhsvxlorigkuxnsxlmhvwqm", "iamtabcpdagicnvdvqcfykonsazrbdivxtczpgqgxjrifukmqjw", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbneryahanhrhkal", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnunl", "tyqcpfvorznmxxdzsuyushueegfrnlzbydnefcfagqnxglkulegntoml", "zlovtmburfkd", "vlzfaamutsfqefpafzffwhvpfw", "bbufxzwpryyakbxuhwwplvdptgybbykqxirsrahsokviihxrawcbgwrktvgacmwtc", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjbzw", "kjundphljswl", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqhlqnapfkcqpdb", "stcphvgxvcaysehvrfdfllwvxf", "epbtkgnnupbbdqgheyaks", "gilhnlfkdz", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwghy", "yswdsvnzucvsdzrmeghevjrfuhoebfedvyvortaxppwqncmspctdcjlwdxfnc", "baizdtmgozykukcrkapsnp", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgcsfjgtwqqubbhjkzmio", "iblyydfzztmtyjmgrxvyjwcobfyxcgyrbtnfhhxswxahze", "tyqcpfyggtmjahhpjzwhohvchyofsxwkehq", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniaymgkdduoabmp", "gpsqlqorcbqffdxlnijgvzvxilnbkynwscuoymqyg", "eidradnaqjwmucbrgtp", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzopnqxxcxshbhdmippldmcuxlvc", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbfmryrbgicgzqecje", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuze", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqandxbuvshebs", "tyqcqqxonxtwakxlrceyknbockvovdwumbjkfrgmudiafbqlflonfavpsrfq", "tyqcpfvorznmxxdzsnkjnrrzpfourbghe", "ziarqmfvzqpqhunfxwfwjtetotozkjaszznbtrvtxarysaxq", "tyqcpfvorznmxfmlzlcuikpxvahtfbfipjcgmeusshufvnirwcopdnvnop", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvdertpdpdjsngezrnyjxotgonuigmqkgipgb", "tyqcpfvorznmxxdzsnkjnrrzpfvfcvufmzzuqrjubsfzp", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqorqffebhoyfvgkspenqpcmvoxpybkjg", "oqojrvinnhlwuqllcsabkpfiusfucpjbsxzzhlgduawaqyp", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjbdchviaxsw", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqnzudhzclswotlbgdffwiekw", "csgadyglxddodloklsegvsbtgtkloklmxkbxxyorcqwybktuzpyeaqasn", "tyqitegmijccjwxuwvchbvuvllmgsdugzxdkiqvnllhmsjyskxpzzds", "tyqcpfvorznigwmavbguxwhunsshdybhzszxvlnpingqgaghkqzeynbbbhgcxeydir", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnunflh", "tyqcpfvorznmbwmhfmudnurhismirfgvojpdmclw", "tyqcpfvjijiwoup", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejemb", "tyqcpfvorznmxxdzsnkjhvabtzucyooctzzrgehlsuyinrrnzjilfpalvqgwoey", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjbdchvie", "tyqcpfvorznmxxxvjwfgcwegpibuifhfxyomnicutaegshpnschktxknqytritr", "tyqcpfvorznetvhiaobezckojwjbeg", "tyqcpfvorzmjabuccipqln", "frutebajqddrtrsmabfmdcgipssymldwscxbbrbpb", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjbdchviotvqi", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjbdchxeyrnlh", "yaxaddctugikoutgcwqsdekghoemtooljxvysnzqqvgpc", "tyqcpfvorznmxxdzsnkjn", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqggjwxdvqesbgrqei", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckoktdj", "mzwjshgbgbdogqbrhfgbjkrqogyynbdwwkdclsbpynlrhxeucuuo", "tyqcpfvorznmxxdzsnkjnrrztrqgpbvvxm", "u", "tyqcpfvorznmxjnsgyirdtzpwywpnrvgadkmdjghlmerbqysaebyge", "tyqcpfvorznmxxdzsnkjnrkjelwoqorxsnyjqdnxfava", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjbdchvqqy", "hcav", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjbdchviof", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwreznx", "yesupowwycvcdbknhrkfyvnpoqtdhcbhybqvhnvgukoohh", "hcvlnbmcrepblcqrvwpfsyevlpyldptubnxkntqhpounxjcw", "lwaxzivycjkanvikqpbrvdvjkaclyuyfitwfycsnfwjg", "tkruiswrcbzsbkwbhhvjzzuuiayqzbxjosjssacislcvbtcojpmyatkfgyx", "ftujoohzvjonlwxwskeydoxpfvbvrdndbhgpuvykif", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqqngbpbdtufkgunalbekxbkpajlgbjtqmswh", "tyqcpfvorznmxxdzsnkjnrrzpewgvvnicz", "tyqcpfvorznmxxdzsnkjnrrzpfgknjxnepksgdzwxsbziwdzsiiyarxhtpp", "jumcvboxaxjfybdlezcjrarolxjtsuweaigkiudusinfmnczdualqzlpwkm", "tyqcpfvoxegnpqesbaugr", "bteznmwyh", "rtbpifxevchngjnlumvtqtpebgtoznvvrzfxqzmcktoxydbstbvukrunnyeqwfd", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejysfrlewzsgukyahggau", "mvrrbfbfwyrxooopgcbwmtjfepejyfrqdkyaqencqqlagoilrtdndfyn", "tyqcpfvorznmyrzwhjxvhooytltygrakvgkqumrimazzhzoueyqnjz", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjbdchviob", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwoyvqczogovza", "tyqcpfvorznmxxdzsnkjnrrzpfgknvfnzshqqfpr", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjghvqg", "zqrnaierpnsigujuxrftdiauazddadqmrwcimxyztwumwzyjcrqvuexnitdecfgo", "xusxbbilivpovzsjvfsdnacipk", "tyqcpfvorznmxxdzkbqgrgeolnwhtvlckmiattpmxwwtmlifnexxbgtpjslwhczrjlhr", "eiuytvdzhcodcrdgthxynurtpsdyguupijjluucpfinrfnsjkdbbzexfmctejlgvd", "tyqcpfudqjrabwwvdvwmsyscnazaxpsjjhetouegipqevvointclztzummwrrbntjlsj", "tyqcpfvobzfvbiuoktjcqjfx", "tyqcpfvorznmxxdzsnamc", "ajqpomnpmsayhelmhfehjbvjaeszqigfqyuixbtyjy", "jpfxangixfavlhcssecxljksydrjxmaldhwpftinywtbmffsmtslefcuddk", "txryxhtutwdrqmpcapbcrlmhzsobueefwfekusmmylr", "etzqiepphjcleaocnwljcdn", "tyqcpfvokfxlbmbzmitacnromkoaoxl", "iddmxxcmwecfutbrbqeihhlnypckofuhkbydmljfemzrvlxsuskxkbgviybzu", "tyqcpfvorznmxxdzwilcfwrdlfqppdnuvgltuoooppwyomtvtggmsfxsxievdlsyame", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvzquhbkvburnhmerkuabrfcjwanzmfenz", "tyqcpqgaus", "zsbcqgctsjdjyfkdvcehawsqzacafwtjmhemfygdahkexvmkqkcehvek", "tyqcpfvorznmxxdzsnkjfesxjshxtlinfjltdfl", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnubpoqoghhgbpew", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjbdcuudsuqq", "tyqcpgwivyfquxqhbkjbioekqbsd", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnudxocavmwpggka", "tyqcpfvorznmxxdzsnkjnrrzpfgknvnlxdokehsjhiohwdeyikeajzipztzhwmxc", "pmpoycdxttisazazsgiswnsnhdmejpjbygvtjhwqydeugbouekvornbeiwmpehikbz", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwcpoxr", "qmgnrjtavzsqtwareroiihendgcvbzbcolvfoanmixxrxdtnmtevvv", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnunix", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetxpdyhmk", "tyqcpfvorznmxxdzsnkjnrfmy", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetaoqgbczdcemzlmqemy", "tyqcpfzmlffhifutomuvfvwaqatopvur", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvdvagddprewvlgx", "ozmyertmnlwybntwxmpynuynhqdbqhosvjwexzqgvdtnvxexxwwwwhqktmzbfkjfn", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckohyof", "rniiqnzbctzeyeeyrxhfzqgbccplsncvtswcrqmevplfzadlulazmpmhdojwaokn", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzakckmtmjxx", "tyqcpfvorznmxxdzsnkjnrrzpfgkhdwienfhpsqbyrvotbgchkkmvk", "tyqcpfvorznmxxdzsnkjnrrumaqtylptffsjzygeumkahutdmalkqtrhtgrsrqcyyti", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjbdchvioncnr", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjbdchvigzpo", "tyqcpfvorznmxxdzsnkjnrrzpfgkeduq", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnccdnakfkhtg", "lhehmbyzcnlwvr", "tyqcpfvojuuprlby", "wds", "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqvegfwmtdcrvdb", "tyqcpfvorznpkeynkmbbyptclmhxxlyjzejqbcatgfrmkbbmxs", "tyqcpfvorznmiqmfrhihxsagizcrwyaukrjwbbgrxdzknq", "ghhlssixrouzbqcpmxzmsnybaygtb", "jndewk", "tyqcpfvorznmxxdzsnkjnrrzpdqanmxattjhgnflnoyynevsxvpbwfmfrnlc"}
	// sw = "tyqcpfvorznmxxdzsnkjnrrzpfgknvqvderckuzdqqgaqejetbnuniwwjbdchviotvdticwxwcliylrpvrokbcguhnfvpd"

	// ps = []string{"sqgepkccqgaavsvxkqfblglvmnivjuuxpfbjgynse", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsvbf", "swmyurvdwtczxckddkvhytrjrrdxtrrarfolpsdlia", "sqgepkccqgaavsvxkqfblglvmnikmnfatfw", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukphjc", "sqgepkccqgaavsvxkqfblglvmnivzrjsqsambood", "sqgepkccqgdbaihqhhwwiidcusssefhdopyokcbzue", "ksly", "sqgepkccqgaavsvxkqfblglvmlkzxfjofb", "hbkffliyrozfjjhpd", "sqgepkccqgaavsvxkqfblglvkrzlwibxof", "sqgepkccqgaavsvxkqfblmzadx", "qpvikftvswcaountcigplsmbyiagkskfchbotvrltihd", "sqgepkccqgaavsvxkqfblglvmnivjuuthpwqlxjuptu", "kihxbjgmxvfzzxbmuwtcf", "sqgepkccqgaavsvxkqfblgldfvygpudgwkzozsx", "sqgepkccqgaavsvxkqfblwcohgwdytzvpzoeconm", "sqgepkccqyqtcub", "vqkopgukuctqicdiag", "yppylmqvewvf", "fatmwrcsmmtlzvqvcsrgxsofwpopwv", "ebwdhmnttshxzeysrv", "sqgegbqjnwmnipcsclwjwmorwntplbhflxminfhlvyf", "sqgepkccqgaavsvxkqfblglvxnqgqqsudbhdy", "u", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsa", "sqgepomrzojevmqnzczhmanxmsnsnujatlvr", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukphjv", "hkdtbrcxdrgeecloejxufrqpebb", "sqgepkccqgaavstjwmb", "sqgepkccqgaavsvxkqfblglvmnivjuuthmtna", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukfcc", "xaufndzl", "sqgepkccqgaavsvxkqfblglvmnivjuuthpdp", "tewcfvkqihfo", "sqgehairusrlgduhbkbojewqbadspihpruxtj", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukphm", "sqgepkccqgaavsvxkqfblglvmnivjit", "sqgepkccqgaavsvxkqfblglvmnivjuutjmwodf", "axasbnkobqqyjefahwvzqggjzwybmlmqlevueusvoh", "jgieouwsfnixqgpiikobrbscox", "sqgepkccqgaavsvxkqfblglvmnivucsbpmpgyhhhumi", "nyhynfy", "umxqxmcbuftfidbswkrqrvhg", "jjozijzxisvoqo", "sqgepkccqgaavsvxkqfblglvmnibouajmfzyrfejvzo", "sqgepkccqgaavjvytdivrnzyfbggbzfoozsubbgm", "dpytkrtegctbemfds", "sqgekwfgdbrhnmcrqg", "sqgepkccqgaavsvxkqfblglvmnivjuutxxfywpgrhis", "sqgepkccqgaavsvxkqfblglvmnivjuuthprgqtkzl", "cvropklw", "sqgepkccqgaavsvxkqfblglvmnivjuuthpivmym", "vapfvcraojrfgxofqwqrcqrxwcxekxohv", "sqgepkccqgaavsvxkqfblglvmnivjuuvt", "sqgepkccqgaavsvxkqfblglvmniedcwxpyipshpves", "sqgebqpohlltyyfmktehtjgnoywyghcndskjaamef", "sqgepkccqgaavsvxkqfbluzuxiepgbsmvgpfzyqgpio", "sqgepkccqgaavsvxkqfblglvmnivjuutadxybix", "xcwswncxpdlmatmu", "sqgepkccqgaavsvxkqrwelnnoflyaxkleaonel", "sqgepkccqgaavsvxkqfblglvmnivjuuthprszaxhplwu", "sqgeywymrhfprzpujddmocujezliyeayx", "senvdtisbjj", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukf", "sqgepkccqgaavsvxknca", "sqgepkccqgzgz", "sqgepkccqgaavsvxkqfblglvmnivjuozlont", "sqkdvlikxmpmybllekmlohqpopjqwajxccyiwn", "qzli", "jdwkwqnarsenpbnk", "quambmljp", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukphjpr", "sqgcpwokxmzocarn", "sqgepkccqgaavsvxkqfblglvmnzmrpimbqc", "sqgepkccqgeilhcwx", "wxxtntufxfo", "sqgepkccqgaavsvxkqfblgnvkcuo", "sqgepkccqgaavsvxkqfblglvmnivjuuyeopqmcqh", "sqgepkccqgaavsvxkqfblglvmnivnifuhe", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukphjpg", "sqgepkccqgaavsvxkqfblglvmnivjuwnbqc", "nvuathteygyxbyw", "sqgepkccqgaavskcgxuaye", "cjzmacumkkvphcodnscesbvogqjobnc", "sqgepkccqnxytejogbawcexeizkz", "sqgepkccqqobkfowjlhhepvrnicxtpotnypotgoo", "kisihvhmjshaoeuydcjmdmr", "sqgepkccqgaavsvxkqfblglvmneupolv", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxufk", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsefs", "sqgepkccqgaamyfuumcosqlgylyrjjklwkltyziyu", "lqnbqfehguivjudiebi", "emosgwifmkhulvbkjfcaeapnjedljznas", "sqgepkccqgaavsvxkqfblglvmnivjuuthpovinh", "sqgepkgnjilsmvrgogqoelpouweguh", "sqgepkccqgaavsvxkqfblgwatinudkufeijq", "iipaeqddhnr", "sqgepkccqgpgtsftlia", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxiviids", "sqgepkq", "splkvpbjehrgu", "sqgepkccqgaavsvxkqfblglvmnivjuutqabsfcudrupn", "idlcfywzugkvrxkhrqkxb", "syumq", "sqgepkccqgaavsvxkqfjznretwocwyfhcskdxt", "sqgepkccqgaavsvxkqfblglvmnivhcdrpnz", "sqgepkccqgaavsvxkqfblglvmnivjear", "ghxmspgqfcbhhrcdeqjq", "sqgepkccqgaavsvxkpkhabroyceeagrmvsokdd", "sqguneiofmumrschzydveietssvoqiamfexjs", "sqgepkccqgaavsvxkqirmypfqgrcwhtuktb", "sqgepkccqgaacgqojpqybbtsrrdq", "gfjihythdgwgeuxg", "sqgepkccqgaavsvxke", "sqhxqfjmqbpxjspbajbkrfuzujemzrwyeazhtdrh", "sqggwcgzawf", "sqgepkccqgaavsvxkqfblglvmnivjuuthpzh", "sqgepkccqgaavrdmcvmnnbdfdmupdublusthoth", "sqgepkccqgaavsvxkqfblglmrlgjttqn", "mbdqytrwursxawe", "sqgepywpzcbqfwxgtvmjni", "pxtgficzrixtflgwspwikfxmfefxcklkzxnvygnpbd", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsytxgobjy", "sqahrucrdeprpjgzmsubljxamwvb", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukz", "iszmimnlazczzkdtgzwldtlhrzlv", "sqgepkccqgdyrwypdqrf", "sqgepkccqgaavjexjg", "sqhcawquicrawyzvfycgdzzandm", "ncxrjxvahehnc", "sqgepkccqgaavsvxkmptcvkpu", "uwpzqjzihrjvjmbyyuizkarabxr", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxyne", "cjttkgu", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxua", "sfvigw", "xvnuecvwyrweuygxmihrznfnly", "sqgepkccqgaavsvxkqfblglvmnivjumwbhbvjsmao", "tpkrdjhsnsovdximwbitoiqipyvetemmufckjnujn", "sqgepkccqgafikmexdjgndsnvxcyhhtokvpne", "achyraqdpwrsdnqgfyzmvimww", "sqgepkjmlelmbzmyyx", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyqafu", "zqegzhjxfnvbfcciygmobkih", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukphna", "sqgepkccqgaavsvxkqcqpsmjkodfpvlwtgfgadkorkow", "pvdawtsayodgthbcbiaodzxlxiy", "sqgepkcczirfnwlhnpmgdthmbvkdlgcwfaqxeduemge", "sqgepkccqdmstkp", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukphjq", "sqgepkccqgazhgruqpjbkihbclv", "sqgepkccqgaavsvxkqfbkfuobbqemgebtckjzrmsc", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxn", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukkc", "embtbtofbotdtopnebtlueulsvrozh", "sqgepawlstgyhrwh", "sqgepkccqgaaiiypzdhxfneteqavuhrgm", "sqgepnysum", "sqgepkccqgaavsvxkqfbxvggyfpwhgrmbvm", "sqgepkcsjujzzlcnmpxxawhmdrkosiaehuzimrskobk", "qmoc", "hiyuyhoyxmzxzlgwkwupriywivczeeoqm", "sqgepkccqgaavsvxkqfblglvmnivjuuthskkp", "ndemejlvbhxgrwnsrcsno", "bqdrotiejwvmieqmkgrpzcutafzljnsyxfgtpfajcn", "sqgepkccqgaavmvcptfqcrfcrsqkfyxizqwndklgisz", "sqgbmohwqpiquytzzuuwmpdfrtmcrms", "kamlzigww", "ybavfctsxtdmhikqumikfe", "yvpwisuimi", "xppzvjzhulgggfnas", "sqgepkccqgaavsvxkqfmhjjlfaujdjrz", "uxekushyyayxigiiocagveeeaipikzngef", "dbhicnyhswznkgjnebc", "zzoljusmmjsvvcdisp", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsynmxvxql", "upzlmvwgdqreshsfr", "sqgepcdhjnmfqngmakkovkybmjx", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukphjb", "sqgepkccqgaavsvxebhdehiuskqfwq", "sdzietgteruddwtvkagieilxxnvcnfyth", "sqgepkccqgaavsouypjrtyslqsuvwsarfllhjwgnh", "loonjqzxolkmqvgulkqeqjzkewc", "sqgepkccqgaavsvxkqfblglvmnivjuuthpoktpzwxpz", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukpf", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukphsk", "sqzvsmsqmlixwttffjwhjusjicydrwdpprfgwrqsx", "jowvlbvqrjpfspobhdtecxtufymgquedrhvfljlx", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyy", "sqfvmv", "sqgepkccqyeqhs", "sqgepkcenz", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsznt", "mnkrnahjfkpqzrdtdckiv", "pgvdkjnptfyib", "bguoghxexneevamefrndeoourp", "ifoofrdllpafslipmwqnbpehqlaixxenexrrgug", "amczahsplpbpqxlkrje", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxuw", "pqxey", "sqgepkccqgaavsvxkqfblglgbqzptstakzfccwfqfbah", "mhqltu", "uzxyyszxsgzaphusapdreusiiucppsmkiskoga", "ikemdypssppdnrnwyuyuywtllqzxwuujjcfk", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxusgqmp", "sqgepzudejcpibmgcdtejuh", "sqgepkccqgaavsvxkqfblglvckycxlsfjyigvyj", "sqgepkccqgaavsvxkonzjkqcyaotfvzqmpa", "sqbzlqdixgikpqydgxyhmpeyz", "dwofsvdikvcyxlkfallimwihalwidm", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukphfy", "ssxirnptwkpqbd", "sqgepkccqgaaotgoxjouqsjzpzdvybamlvlmgnlvr", "sqgepkccqgaavsvxkqfblglvmnivjxudvkpmy", "wkawvjzypfcebbegicanyjulkf", "gliqdugqeymyoqnsgkalevboitop", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukphcr", "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukpuq", "sqgepkccgqalkzmyaofoyhapjrdjlibiglwmehk", "sqgepmgbmeghionessabzdlp", "sqgepkccqgaavsvxkqfyfomtuzjywdvjxqbt"}
	// sw = "sqgepkccqgaavsvxkqfblglvmnivjuuthprsyxukphjpkgttfinykhosswwswlxofjysouakjqkkzaqhvfxzslkheirwuxvyclbekngxjktoekmkdaagxhybknmfoyisnzqfqhyuoydwrkimdgpolqddyhblvmqrvoznomhdjyltkoozmtqtxjmlhqfwcvjhbzrhsgpptsrjukfwmteouzvhjalpngddynpqsrssgpdtngmwvuolbionnlddsnyixcgheucpwkphgbkarrayzmizlawtkmukgi"

	res := suggestedProducts(ps, sw)
	fmt.Println(res)

}
