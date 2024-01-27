using System.Text;


static IList<IList<string>> GroupAnagrams(string[] strs) {
    List<string> strList = strs.ToList();
    List<IList<string>> output = new List<IList<string>>();

    if(strs.Length == 1){ 
        List<string> emptyAnagram = new List<string>(); 
        emptyAnagram.Add(strs[0]);
        output.Add(emptyAnagram);
        return output;
    }

    for (int i = 0; i < strList.Count; i++){
        List<string> FoundAnagrams = new List<string>(); 
        FoundAnagrams.Add(strs[i]);

        if(strs[i].Length != 0){
            for (int j = i+1; j < strList.Count; j++){
                var elem1 = strs[i];
                var elem2 = strs[j];

                if(elem1 != elem2 || elem1.Length == elem2.Length){
                    var elem1Array = elem1.ToArray();
                    var elem2Array = elem2.ToArray();
                    Array.Sort(elem1Array);
                    Array.Sort(elem2Array);
                    elem1 = String.Join("", elem1Array);
                    elem2 = String.Join("", elem2Array);

                    if(elem1 == elem2){
                        Console.WriteLine("FOUND ANAGRAM = " + strs[j]);
                        FoundAnagrams.Add(strs[j]);
                        strs[j] = "";
                    }
                }
            }
            output.Add(FoundAnagrams);
        }
    }
    return output;
}

var result = GroupAnagrams([""]);

foreach (var r1 in result){
    foreach (var r2 in r1){
        Console.Write(r2 + "  ");
    }
    Console.WriteLine();
}
Console.WriteLine("Result = " + result);


