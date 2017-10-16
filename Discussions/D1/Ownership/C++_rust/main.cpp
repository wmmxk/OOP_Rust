#include <iostream>
#include <vector>
#include <string>

using namespace std;


int main()
{
vector<string> v;
v.push_back("Hello");


string & x = v[0];
v.push_back("world");


cout << x << endl;




return 0;		
}
