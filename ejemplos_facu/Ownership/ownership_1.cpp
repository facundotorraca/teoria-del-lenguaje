#include <vector>
#include <iostream>

/*--------------------------------------------------*/
/* C++ al realizar un move no elimina al objeto del */
/* scope en que esta, este sigue existiendo         */
/* con sus elementos alterados                      */
/*--------------------------------------------------*/

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

void funcion_que_hace_algo_con_string(std::string string) {
    string.append("!!!");
    std::cout << "El string movido: " << string << std::endl;
}


void funcion_que_hace_algo_con_numero(Numero numero) {
    numero.imprimir();
}

int main() {
    std::string string("Hola, soy un string");
    funcion_que_hace_algo_con_string(std::move(string));
    std::cout << "Quedo algo? " << string << std::endl;

    Numero numero(5);
    funcion_que_hace_algo_con_numero(std::move(numero));
    numero.imprimir();

    return 0;
}
