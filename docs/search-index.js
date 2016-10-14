var searchIndex = {};
searchIndex["memoization"] = {"doc":"Memoization offers a simple generic enum that allows for variables and\nstructure fields to become memoized. This crate only has 1 instance of unsafe\ncode within itself.","items":[[4,"Memoized","memoization","Magical Enum",null,null],[13,"UnInitialized","","",0,null],[13,"Processed","","",0,null],[11,"new","","Build a new memoized field. The user will pass a lambda function\nthat will initialize the field.",0,{"inputs":[{"name":"func"}],"output":{"name":"memoized"}}],[11,"process","","This will convert an UnInitialized value to a Processed value. When\ncalled on a Processed value this function will PANIC.",0,null],[11,"processed","","Informs user if a field has been Processed.",0,null],[11,"deref","","",0,null],[11,"deref_mut","","",0,null],[11,"borrow","","",0,null]],"paths":[[4,"Memoized"]]};
initSearch(searchIndex);
