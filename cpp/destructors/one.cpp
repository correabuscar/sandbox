//src: https://stackoverflow.com/a/41000041
#include <iostream>

struct Notify {
    ~Notify() { std::cout << "dtor" << std::endl; }
};

struct Parent {
    std::string a;
    virtual ~Parent() {}
};

struct Child //: public Parent 
{
    std::string b;
    Notify n;
};

class Base {
  public:
    virtual ~Base()
    {
    std::cout << "Base dtor" << std::endl;
  };
};

class SubClass// : public Base
{
  public:
  ~SubClass() //override
  {
    std::cout << "SubClass dtor" << std::endl;
  };
};

class SubSubClass: public SubClass {
  public:
  ~SubSubClass() //override
  {
    std::cout << "SubSubClass dtor" << std::endl;
  };
};
int main(int argc, char **argv) {
    //Parent *p = new Child();
    //Child *p = new Child();
    SubSubClass *p = new SubSubClass();
    delete p;
}
