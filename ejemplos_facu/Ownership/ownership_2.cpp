#include <iostream>
#include <vector>

class Numero {

    int* numero;

public:
    Numero(int numero) {
        this->numero = new int(numero);
    }

    Numero(Numero&& otro) {
        this->numero = otro.numero;
        otro.numero = nullptr;
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


/*
    std::vector<Numero*> numeros { new Numero(1),
                                   new Numero(2),
                                   new Numero(3)
                                 };

    std::vector<Numero*> slicing(numeros.begin(), numeros.begin() + 1);

    delete (numeros[0]);

    std::cout << "Length NUMEROS: " << numeros.size() << std::endl;

    slicing[0]->imprimir();
*/
    return 0;
}
