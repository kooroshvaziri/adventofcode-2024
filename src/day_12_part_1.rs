use std::collections::HashMap;

fn repaint_map(colors: &mut Vec<Vec<usize>>, rep: usize, with: usize){
    for i in 0..colors.len() {
        for j in 0..colors[i].len() {
            if colors[i][j] == rep {
                colors[i][j] = with;
            }
        }
    }
}

fn calculate_regions(map: &Vec<Vec<char>>)->usize {
    let map_rows = map.len();
    
    let mut regions = 0;
    let mut colored_map: Vec<Vec<usize>> = Vec::new();
    for i in 0..map_rows {
        let row_len = map[i].len();
        colored_map.push(vec![0; row_len]);
        
        for j in 0..row_len {
            let cur_reg = map[i][j];
            let mut cur_col = colored_map[i][j];

            //check west
            if j>0 && map[i][j-1] == cur_reg {
                if cur_col == 0 {
                    cur_col = colored_map[i][j-1];
                } else if cur_col != colored_map[i][j-1] {
                    //same region different color:
                    let rep_col = colored_map[i][j-1];
                    repaint_map(&mut colored_map, rep_col, cur_col);
                }
            }
            //check north
            if i>0 && map[i-1][j] == cur_reg {
                if cur_col == 0 {
                    cur_col = colored_map[i-1][j];
                } else if cur_col != colored_map[i-1][j] {
                    //same region different color:
                    let rep_col = colored_map[i-1][j];
                    repaint_map(&mut colored_map, rep_col, cur_col);
                }
            }

            if cur_col == 0 {
                regions+=1;
                cur_col = regions;
            }
            colored_map[i][j] = cur_col;
        }
        
    }

    let mut distinct: HashMap::<usize, (usize, usize)> = HashMap::new();
    for i in 0..map_rows {
        let row_len = colored_map[i].len();
        for j in 0..row_len {
            let cur_reg = colored_map[i][j];
            let mut sides = 0;
            //check west
            if j==0 || colored_map[i][j-1] != cur_reg {
                sides+=1;
            }
            //check north
            if i==0 || colored_map[i-1][j] != cur_reg {
                sides+=1;
            }
            //check east
            if j==row_len-1 || colored_map[i][j+1] != cur_reg {
                sides+=1;
            }
            //check south
            if i==map_rows-1 || colored_map[i+1][j] != cur_reg {
                sides+=1;
            }
            
            let (cur_count, cur_side) = distinct.get(&cur_reg).unwrap_or(&(0, 0));
            distinct.insert(cur_reg, (cur_count+1, cur_side+sides));
        }
        
    }
    
    distinct.iter().map(|(_, (c,v))| c * v).collect::<Vec<usize>>().iter().sum::<usize>()
}

fn convert_block_to_vec(source: &str)->Vec<Vec<char>> {
    source.split('\n').collect::<Vec<&str>>().iter().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>()
}

fn main() {
    let source = 
"AAAA
BBCD
BBCC
EEEC";

    assert_eq!(140, calculate_regions(&convert_block_to_vec(source)));

    let source = 
"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    assert_eq!(1930, calculate_regions(&convert_block_to_vec(source)));
    
    let source = 
    "SSSSSOOOOOBFFJJFTFBBBBBBBBPPPPPPPPBBBBBBBBBBBSSDDDDDDCCCCCCCCCCCUUUUUAAAAAAAAAAAAAAAUUUUUUUUUUUUUUUUUUUUUUDDDDDDMMMMMZZFFZZFFFFFFFFFFFFFFFII
SSSOOOOOFFFFFFJFFFFBBBBBBBPPPPPPPPPBBBBBBBBBSSSSSSSSDDCCCCCCCCCUUUUUUUUAAAAAAAAAAAAAUUUUUUUUUUUUUUUUUUUUUUUDDDDDMMMMZZZZFZZFFFFFFFFFFFFFFFII
OOSOOOOOFFFFFFJFFFFBBBBBBPPPPPPPPPPBBBBBBBBBBSSSSSSSDCCCCCCCCUUUUUUUUUUUUAAAAAAAAAAUUUUUUUUUUUUUUUUUUUUUUDDDDDDDDDDZZZZZZZFFFFFFFFFFFFFFFFII
OOOOOOOOFFFFFFFFFFFHBBBBBBPPPPPPPPPBBBBBBBBBBSSSSSSSDCCCCCCCUUUUUUUUUUGGUAAAAAAAAAUUUUUUUUUUUUUUUUUUUUUDVDDDDDDDDDDZZZZZZZZFFFFFFFFFFFFFFFII
OOOOOOOOOFFFFFFFFFFFFBBBBPPPPPPPPPPPBBBBBSSSSSSSSSSSDXXCYCCCUUUUUUUUUUUUUAAXXAAAAUUUUUUUUUUUCUUUUUUUUUUDDDDDDDDDRDHZZZZZZZZFFFFFFFFFFFFFFFII
OOOOOOOOOOFFFFFFFFFFFFFFBPPPPPPPPPPPBPPPBBSSSSSSSSXXXXXCXXCCUUULUUUUUUUUUUAXXXAAAUUUUUUCCCCCCCCCSUUUUUUDDDDDDDDFRRFZZZZZZZZZZZFFFFFFFFFFFFII
OOOOOOOOOOFFFUFFFFFFFFFFIPPPPPPPPPPPBBPPBBSSSSSSSMMMXXXXXXWUUUUUUUUUUUUUUCXXXXXXXWWWUUCCCCCCCCCCSUUUUUDDDDDDDDXFFFFZZZZZZZZZZZFFFFFFFFFFFFII
COOCOOOOOOKFFUFUUUFFFFFIIPPPPPPPPPPPPPPPPPSSSSSSSSMXXXXXXXUUUUUUUUUUUUUUUUUUXXXXXXXWWWCCCCCCCCCCSSSUUUUUDDDDDDFFFKFFZZZZZZZZFFFFFFFFFFFFIIII
COCCCOOOOOKFUUUUUUFFFFFIIIIPPPPPPPPPPPPPPIITSSSSSIXXXXXXXXUUUUUUUUUUUUUUUXXXXXXXXXWWWWWCCCCCCCCSSSSSSSSUUDDDDDFFFFFFZZZZZZZZFFFFFFFFFQQQIIII
CCCCCCCOKOKKUUUUUUFFFFFIIIIPPPPPPPPPPPPPPIIIIISIIIIXXXXXXXUUUUUUUUUUUUUUUXXXXXXXXXWWWCCCCCCCCCGSSSSSSSSSUDDDDFFFFFFFZZZZZZZZMZFFFXFSQQFQQQII
CCCCCCCCKKKKKKUUUUFUUKKIIIIPPPPPPPPPPPPPPIIIIIIIIIIIIIXXXXXURUUUUUUNUNUUXXXXXXXXXXWWWWCCCCCCCCSSSSSSSSSKKDDDDFFFFFFZZZZZZZZZZZXFFXFFFQQQQQII
CCCCCCKKKKKKKKUUUUUUKQKKIIIIIPPPPPPPPPPPPIIIIIIIIIIIIIXXXXXURUKKUUNNNNUXXXXXXXXTXXWCCCCCCCCCCCISSSSSSSSKKKKDDSFFFZZZZZZZZZZZZZXXXXFQQQQQQQQI
CCCCCCCCKKKKKKUUUUUUKKKKKLLIIPPPIPPPPPPPPPIIIIIIIIIIIIXXXXXXRSSSUUNNNNUNXXXXXXZXXWWCCCCCCCCCSSSSSSSSSSSSSKKKDSSZZZZZZZZZZZZZZZXXXXFFGQQQQQQQ
CCCCCCCCCCUKKKUUUUUUUUKKLLLIPPPIIIPPPPPPPIIIIIINIIIIIIIIXXXXSSSSNNNNNNUNXXXXXXZXXXCCCCCCCCCCCSSSSSSSSSSSSKKKSSSZZZZZZZZZZZZFFFFFXXFFFQQQQQQQ
CCCCCCCCCUUUUKUUUUUUULKKLLLLLIIIIIPPPPPPPIIIIKKNIIIINIIIISXXSSSSGGGNNNNNNNXQZZZZZXXXXCCCXCSCCCSSSSSSSSSSSKKKSSSSZZZSZZZZZZZFDFFFFFFFWQQQQQQQ
CCCCCCCCCUUUUUUUUUUUULLLLLLLLLLIIIPPPPPPPKIIKKBNNNNNNNNSSSUXSSJGGGGGNNNNNQQQZZZXZXXXXCCCXXSSSSSSSSSSSSSSSKKSSSSSZSSSZSZZZFFFDFFFFFFFQQQQQQQQ
CCCCCCCCCUUUUUUUOULLLLLLLFLMLLIIIIPPPPPKKKIEEKKNNNNNNNNSSUUSSSJJGGGNNLNNNQQQQZZXXXXXXXXXXXXXXSSSSTSSSSSKKKKKKSSSSSSSSSSZZJFFDFFFFFFFFQQQQQQQ
CCCCCCCCCUJJJJJJUULLLLLLLLLLIIIIIIPPPKKKKKEEKKKNNNNNNNNNSSSSSSSGGGGNNLNNNQQQQQXXXXXXXXXXXXXXXTSSSTSSSSSKKKKKSSSSSSSSSSSZZFFFFFFFFFFFFQQQQQQQ
CCCCCCKKKUUJJJJJJJLLLLLLLLLLLIIIIIIPKKKKKKKKKKKNNNNNNNNNSSSSSSSGGGGGGLLLNQQQQQQQXXXXXXXXXXXXXTSTTTTTSSSKKKKSSSSSSSSSSSSSSSFFFFFFFFFFQQQQQQQQ
CCCCFFFFFFJJJJJJJLLLLLLLLLLLLLIIIIPPKKKKKKKKKKNNNNNNNNNXXXSSMMMGGGLLLLLLNQNQQQQQXXXXXXXXXXXXXTTTTTTTTTSHKKSSSUSSSSSSSSSSSZFFFFFFFFFFQQQQQQQQ
CCCCFFFFFFFJJJJJJLLLLLLLLLLLLLIIIHHPPKKKKKKKKKKKNNNNNNNNXXXSMMLLLLLLLLLNNNNNQQQQQXXXXXXXXXTXTTTTTTTTTTTHKSSSSSSSSSSSSSSSSSFFFFFFFFFQQQQQQQQQ
CCCCFFFFGFFJJJGJJJJJLLLLLKKLLLIIIHHKKKKKKKKKKKKKNNNNNNNXXXXSMMMLLLLLLLLNNNNNQQQQQQXXXXXXXXTTTCCTTTTTTTHHHHHDSSLSSLSSSSSSSSFFFFFFFFFQQEEQQQQQ
CCFGFFFFFFFFGGGJGJJLLLLLUEELILIIIKKKKKKKKKKKIKKUNNNNNNXXXXXXXLLLLLLLLLLLQQQQQQQQQQQXXXXXXXXCTCTTTTTTTTTHHHHHIILLLLSSSSSSHHHFHFFFFFFEEEEEQEQQ
CCFFFFFFFFFFGGGGGJJLLLLLUEESIIIIIIDDKKKKKKKKUUUUNNNNNNXXXXXXZZLLLLLLLLLLLQQQQQQQQQQQXXXXXXJCCCCTTTTTTTTHHIIHIIIWLLSSSQQSHHHHHHFFFFFFEEEEEEQQ
XFFFFFFFFFFFGGGGGGJHLLLLUEEIIIIIIIDDDDKYKKKKKUUUUNUUNNCXXXXXXZZLLLLLLLLBLMBQQQQQQQQQXXCCCXCCCCCTTTTTTTHHHIIIIIWWQQQQQQQSSHHHHHFFFPPFEEEEEEQQ
FFFFFFFFFFFGGGRGGPHHLHHLLEEEIIIIIDDDDDUYYKKKKUUUUUUUWWXXXRXXXKKKKKKLLLBBBBBQQQQQQQCCXXCCCCCCJCCTTTTTTTTHFIIIIIIWWQQQQQQJHHHHHHHJJPEEEEEEEIEQ
FFFFFFFFFCFFFGRRPPHHHHHELEEEEEEIDDDDDUUUYYYKUUUUUUUWWWWWXXXKKKKKKKLLLBBBBBBQQQQQQQQCCCCCCCCCCCCFTTTTFTTFFFIFIWWWWQQQEQQJJHHHHHJJJJJJXEEEEEEQ
FFFFFFFFFFRFRRRRPPHHHHEEEEEEEEEEEDDDDUUUUUKKUUUUUUWWWWWWXKKKKKKKKKLLLBBBBBBQQQQQQQCCCCCCCCCCCCGFFFFFFFTFFFIFIWWWWEEQEQQJJJHJJJJJJJJJJJREQQEQ
FFFFFFFFFFFFFFPPPPPHPEEQEEEEEEEEEEDEUUUUUUUUUUUUUUUWWWWWWKKKKKKKKKKLLBKBBBBBBQQQQCCCCCCCCCCCCCCAFFFFFFFFFIIFIIIWWEEEEQJJJHHJJJJJJJJJJJJJJQEQ
FFFFFFFFFFFFPPPPPPPPPEEEEEEEEEEEEEEEUUUUUUUUUUUUUUUWOXXXXKKKKKKKKKLLLBBBBBBBBBBQQCCCCCCCCCCCCCCCFFCCCFCFFFFFFFFWWEEEEEJJJJJJJJJJJJJJJJJJJQQQ
FKFFFFFFFFFFFPPPPPPPPPEEEEEEEEEEEEEEUUUUUGUUUUUUUWUWOVVXXKKKKKKKKKLBLBBBBBBBBBQQQCCCCCCCCCCCCCCCFCCCCCCFFFFFFFFWEEEWWEJJJJJJJJJJJJJJJQJQJQQQ
KKFFFFFFFFFFFPPPPPPPPPPEEEEEEEEEEHELLLUUUGUUUUUWWWWWOOOXXXWWKKKKKILBBBBBBBBBBOQQQQCCCCCCCCCCCCCCCCCCCGCFFFFKKKAWWWWWWWWJJJJJJJJJJJJJJJJQJQQQ
KKKKKFFFFFKFPPPPPPPPPPEEEEEEEEEEEELLLLLGGGGUUUUWWPWOOCOOOXWWKKKKKIBBBBBBBBBBBOOQQCCCCCCCCCCUUUCCCCCGGGGFFFKKKAAAAAWWWWWWJJJJJJJJJJJJQQQQQQQQ
KKKKKFFFFFKKKPPPPGGGGPIEEEEEEEEEELLLLLLLGRRRUCUPPPPOOOOOOOWWWKKKIIIBBBBBBBBBBBOQOCCCCCCCCCUUUUUCCGGGGGGFCEAAAYAAAAWWAAWWJJJJJJJJJJJJJQRQQQRQ
KKKKKKFFKFKKPPPGGGGTTEEEEEEEEEEEELLLLRRLLRRRRUUNPPPVOIORROWXXXMKKXXBBBBBBBBBBBOOOOOOCCCCCUPUUUUUUGGGGGCCCAAAAAAAAAAAAAWWNJJJJJJJJJJJJQRRQRRQ
KKKKKKFKKKKKKPPGGGGGGGSSSSEEEEEQUUUURRRRRRRRGGGPPPVVPPKRRWWXXXMKXXXBBBBBBBBBBBOOOOOOCCCUUUPUUUUUUUGGGGGCCAAAAAAAAAAAAAANNNNJJJJJJJJJQQRRRDRR
KKKKKKKKKKKKKGGGGGGGGGGSSSEPEEEQUUUUUURUUUUUGGPPPPVPPNRRRRRXXXXXXXXXBBBBBBBBBBBBBOOOOOUUVUUUUUUUYYGGGGRCCAAAAAAAAAAAAAANNNNNNJJJJJQQQRRRRRRR
KKKKKKTKKTKKTTSSGGSGSGGSSSPPFEZUUUUUUUUUUUUUPGPPPPPPPPBRRRRXXXXXXXXXXBXBBBBBBBBBBBOOOOUUUUUUUUUUUUAAGGRCAAAAAAAAAAAAAAAGNNNNNJJJJJQQRRRRRRRR
KKKKTTTTTTTTTTSSGGSSSSSSSSPPFPZUUUUUUUUUUUUPPPPPPPPPMPRRREEXXXXXXXXXXXXBBBBBOBBBOOOOOUUUUUUUUUUUUAAGGGGAAAAAAAAAAAAAAAAANNNNNNJJJJJJRRRRRRRR
KKKKKTWTTTTTTTTSSSSSSSPSPPPPPPUUUUUUUUUUUUUUPPPPPPPPPKRRREXXXXXXXXXXXXXVBBBOOOOOOOOOOUUUUUUUKUUUUAAGVVVVAAAAAAAAAAAAAUUUNNNNNNJJJJJRRRRRRRRR
KKKKKTTTTTTTTTTSSSSSSSPPPPPPPPJUUUUUUUUUUUUPPPPPPPPPPPPPPEEXXXXXXXXXXXXXXFBEEOOOOOOOAUUUAAUUUUUDAAAWYVYAAAAAAAAAAAAAAUUUNNNNNNNRRRRRRRRRRRRR
KKKKTTTTTTTTTTTSSSSSSSPPPPPPPPPURUUUUUUUUUPPPPPPPPPPPRPPPEEEEEXXXUXXXXXXXFFEEOOOOOOOAUUUAAUAUUUAAAAWYYYYYAIAJAAAAAUUUUNNNNNNNNNTRRRRRRRRRRRR
KKKKTTTTTTTTTTSSSSSSPPPPPPPPPPSSUUUUUUUUUUUPSPPPPPPPPPPJPREEEEXXXUUXXXXXCEEEOOOOAOAAAAAAAAAAAUAAAAAYYYYYYQAAAAAAAAUUUUUNNAANNIITTRTRRRRRRRRR
KKKKKTTTTTTSSSSSSSSPPPPPPPPPPPPPPUUUUUUUUUGPPPPPPPPPPPJJJRRRRRXXXOUUUXXXCEEEOOOOAAAAAAAAAAAAAAAAAAAYYYYYYYYYAAAAAAUUUUUUUATTTTTTTTTRRRRRRRRR
KKKKKTTTTTTSSBSSSSSSSPPPPPPPPPPPUUUUGGUUUGGGGGPPPPPJJJJJRRRRRRRRUUUUUSXCCEEEOOOGAAAAAAAAAAAAAAAAAYYYYYYYYYBYYBBAAUUUUUUUUUTTXTTTTTTTTRRRRRRR
KKBKBBTTTTBBBBSSSSSSSSPPPPPPPPPPPUUGGGCGUGGGGGPPPPJJJJJJJRRRRRUUUUUUUUXUEEEEEEEGGAAAAAAAAAAAAAAAAYYYYYYYYYYYBBBBUUUUUUUUUTTTTTTTTTTBBBBRRRRR
KBBBBBBTTBBBBBSSSSSSSSPPPPPPPPPPPPUJGGCGGGGGGGGPXPPJJJJJRRRRRRUUUUUUUUXUVEEEEEEGGGGAAASAAAAAAAAAAYYYYYYYYYYBBBBBUUUUUUUUUTTTTTTTTTTBBBBRRRRR
KKKBBBBBTBBBBBSSSSSSSSPXPPPPPPPPPSSGGGGGGGGGGGPPPPZZZZJJZZNRRRUUUUUUUUUUUEEEEGGGGGIIIIAAEUUUUAAAAYYYYYYYYYYBBBBUUUUUUUUUBBBTTTTTTTTTBBBRRRRR
KKBBBBBBBBBBBBBSSSSSPPPPPPPPPYPSPSSSGSGGGGGGGGPPPZZZZZJZZNNNRRRUUUUUUUUUEEEEEEGGIIIISAAAUUUUWAACYYYYYYYYYYYBBBBBUUUUUUUUBBTTTTTTTTTBBBBRRRRR
KKKBBBBBBBBBRBBBBSSSSSPPPPPPPPSSPPSSSSGGGGGGGGPPZZZZZZZZNNNUUUUUUUUUUUUUUEEEEEEEEEIIIIAAUUUIWAAYYYYYYYYYPBBBBBBBBUUUUUUUBBTTTTTTOOTBBBBRRRRR
OOKKBBBBBBBBBBBBSSSSSSPPPPPPPPSSPPSSSSGGSGGGGGGZZZZZZZPZNNNUUUEEUUUUUUUEEEEEEEEEEIIIIAAIIIIIIIIIIIYYYYYYPBBBBBBBBBBUUUBBBBBBYYYYOONNNBRRRRRR
OOOOOOBBBBBBBBBBBBSSSSPPPKPMPPSSSSSSSSSSSGGGGGGZZZZVVZZNNNNNNEEEUUULUUUULLEEEEEEEIIIIIIIIIIIIIIIIIYYYYPPPBBBBBBBBBBBBBBBBBBBYOYYOONNNNPRRPRP
OOOOOOBBBBBBBBBBBFVSSSPMMMMMMMSSSSSSSSSSSGGGGGGCCZZVZYZNNWWNWEEEUULLULLLLLEEEEEEEIIIIIIIIIIIIIIIIIYYYYQPPBPBBBBBBBBBBBBBBBBBBOYOOOONVVPPPPRP
OOOOOOOBBBBBVVBVTVVSSVMMMMMMMSSSSSSSSSSSGGGGGXCCCCZZZZKKKKKKKKKKELLLLLLLLLEEEEEEEIIIIIIIIIIIIIIPIIIUUPPPPPPBBBKBBBBJJJBBBBBOOOOOOOONVVPPPPPP
OOOOOOOBBBBBVVVVVVQVSVMMMMMMMSSSSSSSSSSSSVGGGXXCMMZZZZKKKKKKKKKKELLLLLLLLEEEEEEEEIIIIIIIIIIIIIIPPPPPPPPPPBBBKBKKKKBJJJBBBBOOOOOOOOONVVPPPPPP
OOOOOOOOOBXVVVVVVVVVVVVMMMMMYSSSSSSSSSSASVVAGXCCMVZMZMKKKKKKKKKKELLLLLLLLLEEEEEIIIIIIIIIIIIIIIIPPPPPPPPPCBBKKKKKKKBJJJBBBBOOOOOOOOOOXXXXPPPP
OOOOOOOOOXXXVVVVVVVVVMMMMMCMSSSSSSSSSSSVVVXGGXMMMVMMMMKKKKKKKKKKQLLLLLLLEEEEIEIIIIIIIIIIIIIIIIIIIPPPPPPPCPBCKKKKKKJJJJJJJJOOOOOOOOOOXXXXXPPP
OOOOOOOOOZXXVVVVVVVVVMTMCMCCSSSSSSSSSSSVVVXXXXMMMMMMMAKKKKKKKKKKLLLLLLLLLLIIIIIIIIIIIIIIIIIIIIIIPPPPPPPPPPBBKKKKKKJJJJJJJJJJJJOOOOOOXXXXXXMM
OOOOOOOOOXXXVVVVVVVVVVVVCCCCCSSSSSSSSSVVVXXXVVMMMMMMMMKKKKKKKKKKLLLLLLLLLKIIIIIIIIIIIIIIIIIIIIIHPPPPPPPPPPBBBKKKKKJJJJJJJJJJJJOOXOOOXXXXXXXM
OOOOOOGOOXXXXXVVVVVVVVVVCCCCCCSSSSSSSSVVVVXXVMMMMMMMMMKKKKKKKKKKLLLLLLLLLKIIKIIIIIIIIIIIIIIIHHHHHPPPPPPPPPQQQQKKKKJJJJJJJJJJJJXXXXXXXXXXXXXM
OOOOOOOOOXXXXXVVVVVVVVVVCCNNCCSSSSSSSSSVVVXXVVMMMMVMMVVVVWWWWWWWLLLLLLLLLKIIKKKIIIIIIIIIIIIIHHHPPPPPPPPPPWOQQQQQQQJJJJJJJJJJJJOSAAXXXXXXXXXM
OOOOOOOOOOXCECCCVVVVVVVCCCNNNCSSSSSSSVVVVVVXVVVMVVVMMVVVVVWWWWWWLLLLLLLLLKKIKKKIIIIIIIIIIIIHHHHPPPPPPPPPXOOQQQQQQQJJJJJJJJJJJJSSWWXXXXXXXXXM
OOOOOOOOOOCCCCCVVVVVCCCCCNNVNSSSSSSVVVVVVVVVVVVMVVVVVVVVVVVLWWWWWLLLLLLLLLKKKCCCCCICIIIIIIHHHPPPPVVVPCCPPOOQQQQQQQJJJJJJJJJJJJSSWWWWXXXXXXXX
OOOOOOOOCCCCCCCCCVVVVCCCCNNNNNNSSSSVVVVVVVVVVNNZZZVVIVVVVVVLLLWWLLLLLLLLOODKKKCCCCCCIIIIBIHHHTTPPOOOCCPPPOOOOOQOXQJJJJJJJJJJJJJSWWWWWXXXXXXX
OOOOOOOOOCYCCCCCCVVVCCCCNNNNNNNSSSSSVVVVVVVVVVNZZZZZIVVVVVVLLLLLLLLLLLLLLDDDDCCCCCCIIIIIIIHHHTTPPFOCCCOOOOOOOOOOXQWWWJJJJJJJJJJSSWWWWXXXXXXR
OOOOOOOOYYYCCCCCCVVCCCCNNNNNNNNNNNVVVVVVVVVVVVNNVVZIIVVVVVVLLLLLLLLLLLLZZDDDZCCCCCCCIIIIIIHHHHHPPFOCCCOOOOOOOOXXXXXXWJJJJJJJJJJSWWWWWWXXXXXX
OOOOOOOYYYYYCCCCCCCCCCCNNNNNNNNNCOVVVVVVVVVVZZNSGVVVVVVVVVVLLLLLLLLLLLLLZDDZZCCCCCCIIIIIIHHHHHHFFFOOOOOOOOOOOOOXXXXXWJJJJJJOSDSWWWWWWWXXXXXX
OOOOOYYYYYYYCCCCCCCCCCCNNNSNNNNNCCVVVVVVVZZVVZFSGGGGVVVVVVVMLLLLLLLLLLLLZZZZZCCCCCCCTIIITTTHHHHFFFOOOOOOOOOOOOOOOXXXXXWXXXOOSDWWWWWWWWWXXXXX
OOOYOYYYYYYYCCCCCSCCCCCCCNNNNNNNCCCVVVVZZZZZZZSSSGGMLVVVVVHMMMMMMLLLLLZZZZZZZZCCCCTTTTITTTTBFFFFFFOOOOOOOOOOOOOXXXXXXXXXXXOOOWWWWWWWWWWXXXXM
OOOYYYYYYYYYYGGCCSSSSCCCNNNNNCCCCCCVVZZZZZZZZZZZZMMMLVVVVMMMRLLBLLFFFFFFFFFFZZCXCCCTTTITTTTBBFFFCFOOOOOOOOOOOOXXXXXXXXXXXXXOOXWWWWWWWWWWWMXM
OOOYYYYYYYYGGGGCCCCSSSCCCNNNNNCCCCKVVNNZZZZZZZZZZZKMMVVVVMMMRLLLLLFFFFFFFFFFZZCCCCZATTTTTTTBFFFCCCOOOOOOOOXXOXXXXXXXXXXXXXWXXXSSSWWWWMMMWMMM
OODBYYYYYYYGGGGGSSSSSSSCCCCCCCCCCCCNNNNNZZZZZZZZZMMMMMMMVMMMMMLOLZFFFFFFFFFFZZZZZZZAATTTTTTBBFCCCOOOOOOOOOOXXXXXXXXXXXXXXXXXXXSSSSWWMMMMMMMM
OBDBBBBBEYSGGGGGSSSSSSSSSCCCCCCCCCNNNNZZZZZZZZZZMMMMMMMMVMMMMMMMMZFFFFFFFFFFZZZZZAAAATTTTBBBFFCCBOOOOOOOOOOOOODDDXXXXXXXXXXXXXSSSSWHHMMMMMMM
BBBBBBBBBGGGGGGSSSSSSSSSCCCCCCCCCCCCZZZZZZZZZZZZMMMMMMMMMMMMMMMMMZZZZZZZZZZZZZZZZAAAAAATTBBBFFBBBIOIZZOZZOOOOOODDXXXXXXXXXXXXXSSHSSHMMHHHMMM
BBBBBBBBGGLGGGGGSSSSSSZZCCCCCCCCCCCCCCZZZZZZZZZZZMMMMMMMMMMMMMMMMZZZZZZZZZZZZZZZAYAAAAATBBBBBBBBBIIIZOOZZZZOODDDDDXXXXDXXXXXXXSSHSHHMMHHMMMM
BBBBBBBBGGGGGGAASSSSSSSSCCCCCCCCCCCCCCZZZZZZZZZZZMMMMMMMMMMMMMMMZZZZZZZZZZZZZZZAAAAAAAATTBBBBBBBBZIZZZZZZZZOODDDDDDDDDDDQQQXXSSSHHHHHHHHMMMM
BBBBBBBBBGGGFFSSSSSSSSCCCCCCCCCCCCCWCCCZZZZZZZZZZZZCMMMMMMMMMMMMZZZZZZZZZZZZZTZAAAAAAAAAAQQBBBBBZZZZZZZZZZZOOODDDDDDDDDQQQQXXSHHHHHHHHHHHYYM
BBBBBBBBBBBBBSSSSSSSSSCCCCCCCCCCCCCWCCCZZZZZZZZZZZZMMMMMMMMMMMMMZZZZZZZZZZZTTTZAAAAAAAAAAQQBBBBBZZZZZZZZZZZZDODDDDDDDDDQQQQQXXHHHHHHHHHHHLYY
BBBBBBBBBBBBBBSSSSSSSSCCCCCCCCCCCCWWCCZZZZZZZZZZZZZYMMMMMMMMMMYZZZZZZZZZZZTTTTTWBAAAAAAAQQQBBBBBWZZZZZZZZZZZDDDDDDDDDDDQQQQQQHHHHHHHHHHHHYYY
BBBBBBBBBBBBSSSSSSSSSSSCCCNNCCCWWWWWYYYZZZZZZZZZZYYYYYMMMKMKMMMKKZZZZZZZZZTTTTXBBBAAAAAAQQQBBBBBBZZZZZZZZZSZDDDDDDDDDDDQQQQQQQHHHHHHHHHHHYYY
EEBBBBBBBBBBSSSSSSSSSSSCCNNCCCZWWZWYYYYYYYZZZZZZZYYYYAMMMKKKMMKKKZZZZZZZZZKKKBBBBBVBAIAQQQQBBBOOZZZZZZZZZZZYDDDDDDDDDDDQQQQQQQHHHHHYHHHHYYYY
EEBBBBBBBBBBSSSSSSSSSSSSCNNCGCZZZZZZYYYYYYZZZZZZZYYYYAAMMKKKKKKKKKZZZZZZZZKZKBBBBBBBBBQQQQQQQBBBBZZXZZZZZZDDDDDDDDDDQDDDQQQQQQBHHHHYYHHHYYYY
EEBBBBBBBBBBSBBSSSSSSSSSSSNNZZZZZZZZYYYYYYYYZZXXYYYYAAAAMKKKKKKKNNNZZZZZZZZZBBBBBBBBJJJQQQQQQQXZZZZXZZZZZZFFFFDDDDDDQDDDDQQQBBBBYYYYHHYYYYYY
EEEBEEBBBBBBBBSSSSSSSSSJJJJJZZZZZZZYYYYYYYYYAXXXYYYYAAAEEKKKKKNKNNNNNZZNZZTBBBBBBBBBJJBQQQQQQQXXXXXXXXXZZFFFFDDDDDQDQDDDDQQQQQBBBYYYYYYYYYYY
EEEEEEBBBBBBBBBSSSSSJCSJJJJPZZZZZZZYYYYYYYXXXXXXXYYYXXDEEEEKKNNNNNNNNNNNNBBBBBBBBBBBBJBSOQQQVQXXXXXJJXXJZFFQQQQQDQQQQQDQQQQQQBBBYYYYYYYYYYYY
EEEEEEZEBBOOBBBSSSJBJCCJJJJJZZZZZZZYYYYYYXXXXXXXXXYXXXXEEEEKKNNNNNNNNNNNNNNBBBBBBBBBBBBOOOQQVXXXXJJJJJJJJFQQQQQQQQQQQQQQQQQQBBBBYYBYYYYYYYYY
EEEEEEZEOBOOBOBSSOJJJJJJJJJJJZZZZZZZCCCAAAAXXAAXXXXXXEEEEEEKKNNNNNNNNNNNNNNBBBBBCBBBBOOOOOQXXXXXYJJJJJJJJJQQQQQQQQYYQQQQQQQQBBBBBBBBBBYYYYYY
EEEEEEEEOOOOOOBOOOJJJJJJJJJJJTZZZZCCCCCCCAAAAAKXXOXOEEEEEEEKKNNNNNNNNNNNNNNNBBBCCCCOOOOOOWYYYYYYYJDDJJJJJJWQQQQQQQYYQQQQQQBBBBBBBBBBBBYYYYYY
EEEEEEEEOOOOOOOOOJJJJJJJJJJJJJJJFGCCCCCCCCCAKKKQXOOOEEEEEEEKKNNNNNNNNNNNNNNNNBCCCCCOOOOOOOYYYYYDDDDDJJJJJJWWQQQQYYYYYYQQQQBBBBBBBBBBRBYYYYYY
EEEEIEOOOOOOOOOOOJJJJJJJJJJJJJZZGGGGCCCCCCCKKKKKKOOEEEEEEEEEECCNNNNNNNNNNNNNNCCCCCOOOOOOOYYYYYYDDDVDDDJJJJWWQQQJYYYYYMMMMMMMMMMJJJJJRBYYYYYK
EEEIIROOOOOOOOOOJJJJJJJJJJJJJJJZGGGGZZCCUUKKKKKKKKOOHPPEEEEEECCNNNNNNNNNNNNNCCCCCCOOOOOOYYYYYYYYDVVVDAJAWWWWQQQDYYYYYMMMMMMMMMMDJJJJJOYKYKKK
ERRRRRRROOOOOOOOOJJJJJJJJJJJJJJZZGZZZCCCUUUKKKKKKKOOHHHPEECCCCCCNNNNNNNNNNNNCCCCYCOYYOOOYYYYYYDDDDVVDAAAAWWQQWDDYYYYYMMMMMMMMMMJJJJJJJYKKKKK
ERRRRRRRRROOOOOOOOJJJJJJJJJJJJJZZZZZZZZZZUUUUKKKIIIIFHPPPECCCKKCNNNNNNNNNNCCCCYYYYYYYOYYYYUUYYYDDDDDAAAAAWWWQWDDDYYYMMMMMMMMMMMJJJJJNYYKKKKK
RRRRRRRRRROOOOOOOOOJJJJJJJJJJJJZZZZZZZZZUUUUUUKGGGIIHHHHPKKCKKKKNCCNNNNCCNNNCCCCYYYYYYYYYYUUUTADDAAAAAAWWWWWWWWWDDDOMMMMMMMMMMMJJJJJJLLLKKKK
RRRRRRRORROOOOOOSOOJJJJJJJJJJVZZZZZZZZZZUUUUUUKGGIIIHHHHPKKKKKKKKKCCCCCCCCNCCCCCCCCCHYHHYUUUUAAADAAAAAAAAAWWWWWDDDDOMMMMMMMMMMMJJJJJJLKKKKKK
ZZRRRRROOOOOSOOOSOOOJJJPJJJJVVVZZZZZZZUUUUUUUUUIIIIIIHHHPKKKKKKKKKKCCCCCCCCCCCCCCCCHHHHHHHUUNAAAAAAAAAAAXAWAWWWDDDDOMMMMMMMMMMMJJJJJJJJKKKKK
SZRXXROOSSOSSSSSSSOOOPJPJJJJVVVZZZXXXUUUUUUUUUUIIIIIIHHKKKKKKKKKKKKCCCCCCCCCCJCCCHHHHHHHHHUUNAAAAAAAAAAAAAAAAAWDDDDOMMMMMMMMMMMJJJJJJJKKKKKK
ZZZXXZOSSSSSSSSSSSSSSPPPPJJJVZZZZZZXXVVVVUUUUUUUIIIIIHBBKKKKKKKKKKKCCCYYYCCJJJCCHHHHHHHHHHCCAAAAAAAAAAAAAADAAIWDDDDOMMMMMMMMMMMJJJJJJJKKKKKK
ZZZZZZOSSSSSSSSSSSPPPPPPPPJVVVVVVZXXXCXXVUUUUUUUIIIIIHHBBKKKKKKKKKKCCYYAYCCJJJCCHHHHHHHHHHCHRRAAAAAAAAAAAAAAAAADDDDOMMMMMMMMMMMJYJJJJJKKKKKK
ZZZZZZZSSSSSSSSSSSPPPPPPPPPPPVVVVZXXXXXXVUUUUIIIIIIIIIZZKKKKKKKKKKKKCYYYYYCJJJCCCHHHHHHHHHHHHRRAAAAAAAAAAAAAAAIIIDCCLOOMMMMMJJYYYKKYKKKKKKKK
ZZZZZZZZSSSSSSSSSSPPPPPPPPPVVVVVVVVXXXXXUUUUUIIIIIIIIIZZKKKKKKKKKKKKYYYYDDJJJJJJJIIIHHHHHHHHHRRAAAAAAAAAAAAMMLLLLLLLLOOMMMMMYYYYKKKKKKKKKKKK
ZZZZZZZZASSSSSSSJSPPPPPPPPPPVVVVVVXXXXXXXXUUUUIIIIIIIIIZZKKKKKKKEEDYYYYYYDJJJJJJJJIIJHHHHHHHHRRRRRARARRARRAAMLLLLLLLLOOOOOJYYYYYKKKKKKPKKKKK
ZZZZZZZZZZSSSSSSPPPPPPPPPPVVVVVVVVVXXXXXXXXUUUUIIIIHIIZZZZZZKKKKEEDYYYYYDDDJJJJJJJIIJJHHHHHHHRRRRRRRRRRRRRRAALLLLLLLLOOOOOJYYYYFFFKKPKPPPKWK
ZZZZZZZZZZZUSDSSSPPPPPPPPVVVVVVVVXXXXXXXXXXUUUUUUIIIIIZZVVZKKEFEEEYYYYYYDJJJJJJJJJJJJHHHHHHHHFFFRMMMMMMRRRRRLLLLLLLLLOOOOOJYFFFFFEKPPPPPPKWK
ZZZZZZZZZZZZDDDSSPPPPPOPPPPVVVVVVXXXXXXXXXXXUUKUUIIIIQVZVVVTKEEEEEEYYYYYDJJJJJJJJJLJHHHHHTHHHFFUMMMMMMMMMMRRRRLLLLLLLLOOOOOOOFFFFFFGPEEPPWWK
ZZZZZZZZZZZZZDDDSPPPPPOOEPVVVVVVVVXXXXXXXXXXUKKKKKIIIQVVVVVTTEEEEEEETTYYYJJJJJJJJJJHHHHHHHHHHFFUMMMMMMMMMMRRLLLTLLDLARAOOOOOOHHFFFXPPEEPPPPK
ZZZZZZZZZZZZDDDDDPPPPOOOOOOOOVVVVVXXXXXXXXXUUKKKKKKIQQVVKVTTTEEEEEEETTTTJJJJJJJJJJZZHHHHHHHHFFFMMMMMMMMMMMMRJTTTTTAAAAAOOOOOOOHFFFFFPPPPAPPA
ZZZZZZZZZZZZDDDDDPPPPOOOOOOOOVVVVXXXXXXXXXUUKKKKKKKIIKKAVVVVTTEEEEEETTTTTTJJJJJJJJZZZHHHHHFFFXXXXXXXXXMMMMMJJTTTTTTAAAAAAOOOOOHFFFFFPPPAAAAA
ZZZZZZZZZZZIZDDDDDDPPOOOOOOOOVVVVBXXXXXXXPKKKKKKKKKKKKAAVTTTTTTTEEEETTTUUUUUUJJJJJJZZHHHHHFFFXXXXXXXXXXXXXXXJJJTTTTAAAAAAOOHOOHFFFMPPPSPPPAA
BZZZZZZZZZZZZZDDDDDPDOOOOOOOOOOVVBLXXXXTXXKKKKKKKKKKKKKKTTTTTTTTTEEETTUUUUUUFJJJJJJJHHYHHHHFXXXXXXXXXXXXXXXXJJTTTTTAAAAAAOAHHHHHPFFPPPPPPPAA
BZZZZZZZZZZZZZDDDDDDDDOOOOOOOBBBBBLXXXXKKKKKKKKKKKKKKKKKTZZTTTTTTEEETUUUUUUUJJJJJJCCCCFFHHFFXXXXXXXXXXXXXXXXJTTTJTTAAAAAAAAAAAAPPPPPPPPPPPAA
BZBBBZWWZZNZDZDDDDDDDDDDOOOOOLLBBBLLXXXXRRRRKKKKKKKKKKKKTZZTTTTTTTTTTUUUUUUUUJJJJJCCCCFFFFFFXXXXXXXXXXXXXXXXTTTTJTTAAAAAAAAAADPPPPPPPPPPPPPA
BBBBBBBBBWWNDDDDDDDDDDDDODDOOLLLLLLXXXXRRRRNNNNNKKKKKAAZZZZTTTTTTTTUUUUUUUUCCCJJJVVVFFFFFIIIXXXXXXXXXXXXXXXXJTTTJJTAAAAAAAAAAPPPPPPPPPPPAAAA
BBBBBBBBWWNNDDDDDDDDDDDDDDDDDDLLLLLLXRRRRRRNNNNNKKKKKKZZZZZZTTTTTTTUUUUUUUKKVCCJJVVVCFFFFFIIXXXXXXXXXXXXXXXXTTJJJJTAAAAAAAAAAAPPPPPPPPPPPPAA
BBBBBBBWWWNNDNDDNDDDDDDDDLLLLLLLLLLLLRRRRRRNNNNNKKKKKZZZZZZZTTTTKKTKAUUUUKKKKCCCJVVVCCFFCFIIXXXXXXXXXXXXXXXXTTJJJJTTTAAAAAAAAAZPPPPPPPPPPPAA
BBBBBBBRWWNNNNNNNNDDDDDDLLLLLLLLLLLLLLRRRRRNNNNNKKKZZZZZZZZZZNNNNKKKUBUUUKKKKCVVVVVVCCCFCCCIXXXXXXXXXXXXXXXXTTTJJKTTTAAAAAAAAAZZPPPPPPPPPPPA
BBBBBBBBBNNNNNNNNDDDDLLLLLLLLLLLLLLLLLRRRRRNNNNNKZZZZZZZZZZZZNNNKKKKUUUUUKKKKCVVVVVVCCCCCCCIXXXXXXXXXXZZZZKKKKKJJKKTTTTAAAAEAAZZZZPZPPPPPPPA
BBBBBBBVNNNNNNNNNDDDDDDLWWLLLLLLLLLLRRRRRRRNNNNNZZZZZZZZZZZZZNNNNKKKUUKUKKKKKCVVVVVVCCCCCCIIIIIIZZZZZZZZZZKKKKKKKKKTTTTTAAAEAZZZZZZZZZPPIITA
BBBBBBBBBLNNNNNNNDDDDKKWWWLLLLLLLLLLLRRRNNNNNNNNNNZZZZZZZZNZNNKNNLKKUKKKKKKKKCVVVVVVCCCCCCCVILIIZZZZZZZZZZKKKKKKKKKKKTTTAAAEEGGZZDDDDZPPITTA
BBBBBBBBBLLLNNNNNNDKKKKKWWWWWLLLLLLLRRRRNNNNNNNNNNNNZZZZZNNNNKKKKKKKKKKKKKKKKKVVVVVVKCCCCCCCLLLIZZZZZZZZZZKKKKKKKKYYTTTTTAEEGGGZDDDDTZZZTTTT
BBBBBBBBLLLLNNNNNNKKKKKKWWWWLLLLLLLLRRRRNNNNNNNNNNNNZZZZZZNNNNKKKKKKKKKKKKKKKKVVVVVVVVVVCCCCCCLLIITZZZZZZZKKKKKKKKYYYYTTTGGGGGGDDDDDTTTTTTTT
BBBBBBOWLWLLLNNNKNKKKKKKWWWWLLLLLLLLLRRRNNNNNNNNNNNNZZZZZZNNNNKKKKKKKKKKKKKKKKVVVVVVVVVVKKCQQFLLLTTZZZZZZZKKKKKKYYYYYTTTGGGGGWWWDDDDDDTHTTTT
BBBBBLWWWWWNNNNNKNKKKKKWWWWLLLLLLLWLLRRRNNNNNNNNNNNNZZZZZZZNNNNKKKDWWWKKKKKKKRVVVVVVVVVVKKVQQFFFFCCZZZZZZZKKKKKYYYYYTTTTGGGGGGWDDDDDDDTTDLTT
BBBBBBWWWWWNCNNNKKKKKKKKWWWWLWLWLLWLLRRRNNNNNNNNNNNNZZZZZZZNNNNNKKDWWWKKKKKRRRRRRQVVVVVVKKKQQFQFFFCZZZZKKKKKKKKKKYYYYTTTGGGGGGGDVDDDDDDDDDDD
BBBBBBBBWWWCCKKKKKKKKKKKWWWWWWWWLWWWRRRRNNNNNNNNNNNNZZZZZZZNNNNNOWWWWWKKKRRRRRRRRRKKKKKKKKKQQQQQCCCCCCKKKKKKRKKKRYYYYTGTGGGYWDDDDDDDDDDDDDDD
BBBBBBBBHWWHKKKKKKKKKKKKKWWWWWWWWWWWRRRRRNNNNNNNNNNNEEEEZZNNONNNOOOWWWKKKKRRRRRRRRKKKKKFKKKQQQQQKCCWWCKKKKKRRRRRRJXXXTGGGGGYYDDDDDDDDDDDDDDD
FBBBBBHHHHHHKKKKKKKKKKKKKWWWWWWWWWWWWRRRRNNNNNNNNNNNEEEEZZNNOOOOOOOWOWKKKRRRRRRRKKKKKKKFFFFQQQQQQWWWCCCKKKRRRRRRRXXXXXGGGGYYODYYDYYDDDDDDDDD
BBBBBBBBHHHHHKKKKKKKKKKKKWKWWWWWWWWWWWRRRNNNNNNNNNNNOOEEEZNNNOOOOOOOOWWUKKCCRRRRKKKKKKKKQQQQQQQQWWWWCCCCCKRRRRRRXXJJJYYGGYYYYYYYYYYYYDDDDDDD
BBCBOBECCCHHKKKKKKKKKKKKKKKWWWWWWWWWWWRRRRRRRRRROOOOOOEEEZZNSOOOOOOOOUUUCCCCRRRIIIKKKKKKQQQQQQQQQQCWWCCCCRRRRRRRXXIYYYYGGYYYYYYYYYYYYYYYYYDD
CCCCCCCCCCCKKKKKKOKKKKKKKKKWWWWWWWWWWWRRRRRRRRROOOOOEEEEEZZSSOOOOOOUUUUCCCCCCCIIIKZKKKKKKKQQQQQQQQCCCCCCWWWWRRRXXXIYYYYYGYYYYYYYYYYYYYYYYYDD
CCCCCCCCCCKKKKKKGGGGKKKKKKKWWWWWWWWWWWRRRRRRRRROOOOOEEESSSZSSOMMOOOOCCCCCCCCCIIIIKKKKKKKKIQQQQQPPQQQCCCCCWWRRRXXXIIIYYYYYYYYYYYYYYYYYYYYDDDD
CCCCCCCCCCCKKKKKGGGGKKKKKKWWWWWWWWEWWRRRRRRRRRRROOOOOOSSSSSSSMMOOOOOOCXCCCCCCIISSSLLLKKKIIQIIIICCCCCCCCCCWWRRRXXXIIYYYYYYYYYYYYYYYYYYYYYDDDD
CCCCCCCCCCCCCKKGGGGGGGKKKKWKWWWEWWEEEGGRRRRRRRUOOOOOOOSSSTSSSMMOOOOOOOOOCCCCCIISSSSLLKKIIIIIIIIICCCCCCCCCCCXXXXXXXIIYYYYYYYYYYYYYYYYYYYDDDDD
CCCCCCCCCCCCCCTGGGGGGGGGKKKKEEWEEEEEEGPRPPPRRUUOOOOOOOSSSTSSSMOOOOOOOOOOOCCCCIIISSSSSIIIIIIIICCCCCCCCCCCCCCXJXXXIIIIIYYYYYYYYYYYYCCYYYYDDDDD
CCCCCCCCCCCCGTTGGGGGGGGGKKEEEEEEEEEEEPPPPPPRUUOOOOOOOOOOSOSSSOOOOOOOOOOOOOIICCISSSSSIIIIIIIIICICCCCCCCCCCCCJJJXXIIIIIIIYYYYYYYYYCCCYYYDDDDDD
CCCCCCCCCCCGGGGGGGGGGGGGKKEEEEEEEEEEEEEPPPPRUUOZOOOOOOOOSOSSSOOOOOOOOOOOOOOIIIISSSUSIIIIIIIIIIICCCCCCCCCCCJJJJJJIIIIIIIYYYYYYYYYCCYYYYYDDDDD
CCCCCCCCCCCCGGGGGGGGGGGGKKKEEEEEBEEEEPPPPPPPPOOOOOOOOOOOOOOUOOOOOOOOOOOOOOOOIIIIVVIIIIIIIIIIIIIIICICCCCCCCCJJYYJYIIIIIIYYYYYYYYYYYYYYYDDDDDD
CCCCCCCCCCCCGGGGGGGGGGGGGKKKEEEEEEEEEPPPPPPPPOOOOOOOOOOOOOOUOOOOOOOOOOOOOOOOOOIIIIIIIIIIIIIIIIIIIIICCCCCCCCYYYYYYIIIIIIIYYYYYYYYYYYYYDDDDDDD
CCCCCCCCCCCCGGGGGGGGGGGGGKKKKKKKEEEECPPPPPPPOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOIIIIIIIIIIIIIIIIIIIIICIICCCCCYYYYYYYYYYYYYYYYYYYYYYYYYYYDDDDDD
CCCCCCCCCCCGGGGGGGGGGGGGGKKKKKKKEEECCCCPPPPIOOOOOOOOOOOOOOPOOOOOOOOOOOOOOOOOGGGGIITIIIIIIIIIIIIIIIIIIICCCYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYDDY";

    assert_eq!(1434856, calculate_regions(&convert_block_to_vec(source)));
}
