#include <stdio.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

void gerarNumeros(int *vetorSorteio, int max);
void arrumaVetor(int *C);
int buscaBinaria(int indiceSorteio, int *vetorSorteio);
int gamePessoa(int max,int *vetorSorteio, int indiceSorteio);

int main(void){
  
  int vetorSorteio[20];
  int max;
  int indiceSorteio;
  int tentativasComputador = 0;
  int tentativasJogador = 0;

  printf("Escolha o limite maximo dos valores a serem sorteados (Esse valor deve ser maior que 20 e menor que 350)\n");
  scanf("%d", &max); 

  while(max <= 20 || max > 350){
    printf("Valor menor que 20 ou maior que 350. Escreva novamente\n");
    scanf("%d", &max);
  }

  printf("Gerando 20 valores aleatorios de 1 até %d:\n\n", max);
  srand(time(NULL));

  gerarNumeros(vetorSorteio, max);
  arrumaVetor(vetorSorteio);

  printf("Vetor Sorteio: ");
  for (int i=0; i < 20; i++){
    printf("%d ", vetorSorteio[i]);
  }
  printf("\n \n");
  
  indiceSorteio = rand() % 20;

  printf("Sua Vez: \n \n");
  tentativasJogador = gamePessoa(max,vetorSorteio,indiceSorteio);
  printf("\n");

  printf("Vez do Computador: \n \n");
  tentativasComputador = buscaBinaria(indiceSorteio, vetorSorteio);
  printf("\n");
  
  if(tentativasJogador > tentativasComputador){
    printf("Computador ganhou de %d tentativas cotra %d tentativas", tentativasComputador, tentativasJogador);
  }else if(tentativasJogador < tentativasComputador){
    printf("Jogador ganhou de %d tentativas cotra %d tentativas", tentativasJogador, tentativasComputador);
  }else{
    printf("Jogo Empatou");
  }
  
  return 0;
}

  void gerarNumeros(int *vetorSorteio, int max){
    int vetorValores[max];
    int indice;

    for(int i = 0; i < max; i++){
      vetorValores[i] = i+1;
    }

    for(int i = 0; i < 20; i++){
      indice = rand() % (max-(i+1));

      vetorSorteio[i] = vetorValores[indice];
      for(int j = indice + 1; j < max; j++){
        vetorValores[j-1] = vetorValores[j];
      }
    }
  }

  void arrumaVetor(int *C){
    int maior;

    for(int i = 0; i < 20; i++){
      for(int j = i+1; j < 20; j++){
        if(C[i] > C[j]){
          maior = C[j];
          C[j] = C[i];
          C[i] = maior;
        }
      }
    }
  }

  int buscaBinaria(int indiceSorteio, int *vetorSorteio){
    int acertoComputador = 0;
    int cont = 1;
    int indiceMax = 19;
    int organizaBin = 10;
    int indiceBin = indiceMax - organizaBin;
  
    printf("Tentativa %d: %d\n", cont, vetorSorteio[indiceBin]);
      
    while(acertoComputador < 1){
      if(indiceBin == indiceSorteio){
        acertoComputador = 1;
        break;
      }else{
        if(indiceBin > indiceSorteio){
          if(organizaBin != 1){
            organizaBin = organizaBin/2;
          }
          indiceBin -= organizaBin;
          cont++;
          printf("Tentativa %d: %d\n", cont, vetorSorteio[indiceBin]);
        }else{
          if(indiceBin == 14){
            indiceBin = 17;
            cont++;
            printf("Tentativa %d: %d\n", cont, vetorSorteio[indiceBin]);
          }else{
            if(organizaBin != 1){
            organizaBin = organizaBin/2;
            }
            indiceBin += organizaBin;
            cont++;
            printf("Tentativa %d: %d\n", cont, vetorSorteio[indiceBin]);
          }
        }
      }
    }
  
    return cont;
  }

  int gamePessoa(int max,int *vetorSorteio, int indiceSorteio){
    int cont = 0;
    int acertoPessoa = 0;
    int tentativa;
    while(acertoPessoa < 1){
      while(1){
        cont++;
        printf("Tentativa %d \nDigite um numero entre 1 e %d\n", cont, max);
        scanf("%d", &tentativa);
        if(tentativa < 1 || tentativa > max){
          printf("Numero fora do liimte. Tente novamente.\n");
        }else{
          break;
        }
      }
      if(tentativa == vetorSorteio[indiceSorteio]){
        printf("Voce Acertou na %d tentativa!\n", cont);
        break;
      }else{
        if(tentativa < vetorSorteio[indiceSorteio]){
          printf("Voce errou! Muito baixo o valor.\n \n");
        }else{
          printf("Voce errou! Muito alto o valor.\n \n");
        }
      }
    }
    
    return cont;
  }
