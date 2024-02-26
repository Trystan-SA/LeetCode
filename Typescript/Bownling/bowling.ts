type possibleScores = number | '/' | '-' | 'X';
type Pins = Array<possibleScores[]>

let score: possibleScores[] = [5,4, 4, '/', 7,'-', 'X','-', 'X', '-', 'X', '-', 5,3, 6, '/', 4, '/', 'X', 'X', 'X'];



function GatherThrowIntoPins(scores: possibleScores[]): Pins {
    let result: Pins = [];

    scores.forEach((score, i)=>{
        if(i%2 === 1){
            result[result.length-1].push(score)
        } else {
            result[result.length] = [score]
        }
    })

    return result;
}


function PinToNumber(pin: possibleScores): number {
    if(pin === "-") return 0
    if(pin === '/') return 10
    if(pin === 'X') return 10
    else return pin
}

function GetPinsScoring(pins: Pins): number[] {
    let scoring: number[] = [];

    pins.forEach((pin, i)=>{
        let [Roll1, Roll2] = pin;
        if(Roll2 === '/') scoring[i] = 10 + PinToNumber(pins[i+1][0])
        else if(Roll1 === 'X') {
            let bonusScore = 10;
            if(pins[i+1]){
                if(pins[i+1][0] === 'X'){
                    if(pins[i+2]) bonusScore += PinToNumber(pins[i+2][0])
                    bonusScore += 10
                }
                else bonusScore += PinToNumber(pins[i+1][0]) + PinToNumber(pins[i+1][1])
            }
            scoring[i] = bonusScore
        }
        else {
            scoring[i] = PinToNumber(Roll1) + PinToNumber(Roll2);
        }
    })
    return scoring;
}

function GetTotalScoring(pins: number[]): number {
    let total = 0;
    pins.forEach((pin, i)=>{
        total += pin
    })
    return total;
}


function CalculateBowlingScore(score: possibleScores[]): number {
    let pins = GatherThrowIntoPins(score);
    let scoring = GetPinsScoring(pins);
    let total = GetTotalScoring(scoring);

    console.log("")
    console.log("PINS =", pins);
    console.log("PINS SCORING =", scoring);
    console.log("TOTAL =", total);

    return 0
}


console.log("Bowling Total Score =", CalculateBowlingScore(score)); // 134