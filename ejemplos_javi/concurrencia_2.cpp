#include <thread>
#include <iostream>
#include <mutex>
#include <memory>

class Number{
    int number;
    std::mutex mtx;

    public:
        explicit Number(int x) : number(x), mtx(){}

        void add(int x){
            std::unique_lock<std::mutex> lock(mtx);
            number += x;
        }

        bool lessthan(int x){
            std::unique_lock<std::mutex> lock(mtx);
            return (number < x);
        }

        void show(){
            std::unique_lock<std::mutex> lock(mtx);
            std::cout << number << std::endl;
        }
};

void add_to_100(std::shared_ptr<Number> num, int n){
    while (num->lessthan(100)){
        num->add(1);
        //std::cout << "Thread " << n << std::endl;
        //num->show();
    }
}

int main(){
    std::shared_ptr<Number> x(new Number(0));
    auto y = x;
    auto z = x;

    std::thread h1(&add_to_100, x, 1);
    std::thread h2(&add_to_100, y, 2);

    h1.join();
    h2.join();

    z->show();

    return 0;
}