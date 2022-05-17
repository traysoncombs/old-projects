#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
	float *p;
	int size_t;
  int capacity;
} farray;

void RemoveTrailingFZeros(farray *arr);

int SyntheticDivision(farray *degrees, farray *depressed, float factor);

void GetPossibleZeros(float p, float q, farray* buffer);

void FactorInt(int fact, float* buffer);

float FindRealZero(farray *degrees, farray *possibleZeros, farray *depressed);

void FindRealZeros(farray *degrees, farray *possibleZeros, farray *depressed, farray *factors);

farray *NewFArray(int size);

void FArrayCopy(farray *dest, farray *src);

void AddFElement(farray *arr, int index, float value);

void FreeFArray(farray *arr);

int main(void) { 
  int degree;
  printf("Enter number of degrees: ");
  scanf("%d", &degree);
  farray *degrees = NewFArray(degree+1);
  for (int i = 0; i <= degree; i++){
    printf("a%d: ", i);
    scanf("%f", &degrees->p[i]);
    degrees->size_t++;
  }
  float p = degrees->p[0];

  float q = degrees->p[degree];
  farray *possibleZeros = NewFArray(abs((int)(p*q)));
  GetPossibleZeros(p, q, possibleZeros);
  farray *depressed = NewFArray(degree+1);
  farray *factors = NewFArray(10);  // fix
  FindRealZeros(degrees, possibleZeros, depressed, factors);
  for (int i = 0; i < factors->size_t; i++){
      printf("Factors: %f\n", factors->p[i]);
  }
  for (int i = 0; i < depressed->size_t; i++){
    printf("Depressed: %f\n", depressed->p[i]);
  }
  return 0;
}

void GetPossibleZeros(float p, float q, farray* buffer) {  /* needs to get factors of p and q, remember these are both positive and negative*/
  float Pfactors[abs((int)p*2)];
  float Qfactors[abs((int)q*2)];
  FactorInt(abs((int)p), Pfactors);
  FactorInt(abs((int)q), Qfactors);
  int counter = 0;
  for (int Qindex = 0; Qindex <= (sizeof(Qfactors)/sizeof(int)); Qindex++){
    if (Qfactors[Qindex] == 0) return;
    for (int Pindex = 0; Pindex <= (sizeof(Pfactors)/sizeof(int)); Pindex++){
      if (Pfactors[Pindex] == 0) break;
      AddFElement(buffer, counter++,  Pfactors[Pindex] / Qfactors[Qindex]);
      AddFElement(buffer, counter++,  (Pfactors[Pindex] / Qfactors[Qindex])*-1);
    }
  }
}

void FactorInt(int fact, float *buffer) {
  int count = 0;
  for (int i = 1; i <= abs(fact)/2; i++){
    if (fact%i==0){
      buffer[count++] = i;
    }
  }
  buffer[count++] = fact;
}

int SyntheticDivision(farray *degrees, farray *depressed, float factor) { 
  int arrLength = degrees->size_t;
  int remainder = factor*degrees->p[arrLength-1];
  remainder += degrees->p[arrLength-2];
  depressed->p[arrLength-2] = degrees->p[arrLength-1];
  depressed->size_t++;

  for (int i = arrLength-3; i >= 0; i--){ // fucking hell
    depressed->p[i] = remainder;
    depressed->size_t++;
    remainder *= factor;
    remainder += degrees->p[i];
  }
  return remainder;
}

float FindRealZero(farray *degrees, farray *possibleZeros, farray *depressed){
  for (int i = 0; i< possibleZeros->size_t; i++) {
    if (SyntheticDivision(degrees, depressed, possibleZeros->p[i]) == 0) {
      return possibleZeros->p[i];
    }
  }
  printf("aborting...");
  abort(); // fix 
}

void FindRealZeros(farray *degrees, farray *possibleZeros, farray *depressed, farray *factors) {
  int count = 0;
  farray *degs = NewFArray(degrees->size_t);
  FArrayCopy(degs, degrees);
  int size = 4; // doens't really matetr what this is as long as it starts higher than 4
  while (size > 3){  // if depressed equation has 3 numbers it is a quadratic which may have imaginaries
    depressed->size_t = 0;
    AddFElement(factors, count++, FindRealZero(degs, possibleZeros, depressed)); // degrees is much too big, RemoveTrailingFZeros might not be working
    RemoveTrailingFZeros(depressed);
    degs->p = realloc(degs->p, depressed->size_t*sizeof(float)); // allocating too much somehow, just make new array
    degs->capacity = depressed->size_t; // won't work until comment above is done
    FArrayCopy(degs, depressed);
    size = depressed->size_t;  // size of depressed equation
  }
  RemoveTrailingFZeros(factors);
}

farray *NewFArray(int size){
  farray *arr = malloc(sizeof(*arr));
  arr->p = malloc(size * sizeof(float));
  arr->size_t = 0;
  arr->capacity = size;
  return arr;
}

void AddFElement(farray *arr, int index, float value){
  arr->p[index] = value;
  if (arr->p[index] != 0){
    arr->size_t++;
  }
}

void FArrayCopy(farray *dest, farray *src){
  for (int i = 0; i < src->capacity; i++){
    dest->p[i] = src->p[i];
  }
  dest->size_t = src->size_t;
}

void RemoveTrailingFZeros(farray *arr){
  int zcount = 0;
  farray *temp = NewFArray(arr->size_t);
  for (int i = 0; i < arr->size_t; i++) {
    if (arr->p[i] != 0) {
      temp->p[i] = arr->p[i];
      temp->size_t++;
    }
  }
  FreeFArray(arr);
  arr = NewFArray(temp->size_t);
  FArrayCopy(arr, temp);
}

void FreeFArray(farray *arr){
  free(arr->p);
  free(arr);
}