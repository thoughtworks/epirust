//
// [100, 200, 400] 12
//
//   [12, 24, 48]
//
//   [[0, 4, 8]
//   [5, 0, 19]
//   [16, 32, 0]]

const generate_matrix = (populations, percentage) => {
  let total_population = populations.reduce((x,y) => x+y);
  return populations.map((p1, i1) => {
    let total_commuters = (p1/100)*percentage;
    return populations.map((p2, i2) => {
      if (i1 === i2) { return 0;}
      return Math.ceil((p2/(total_population - p1)) * total_commuters);
    })
  })
}

const mumbai_population = [
  185014,
  127290,
  166161,
  346866,
  393286,
  529034,
  360972,
  377749,
  599039,
  307581,
  557239,
  823885,
  748688,
  902225,
  807720,
  411893,
  622853,
  941366,
  463507,
  562162,
  431368,
  691229,
  743783,
  341463];


console.log(generate_matrix(mumbai_population, 25));
