#include <iostream>

class Numero {

    int* numero;

public:
    Numero(int numero) {
        this->numero = new int(numero);
    }

    Numero(Numero&& otro) {
        this->numero = otro.numero;
        //otro.numero = nullptr;
    }

    void imprimir() {
        std::cout << "EL numero es: " << *this->numero << std::endl;
    }

    ~Numero() {
        delete this->numero;
    }
};


Numero& get_random_numero() {
    Numero a(3);
    return a;
}

int main() {
    Numero& number = get_random_numero();
    number.imprimir();

    return 0;
}