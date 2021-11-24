var min = 1;
var max = 101;
var randNum = Math.floor((max - min) * Math.random() + min)


function makeGuess() {
    var inputVal = parseInt(document.getElementById("input").value);
    if (randNum < inputVal) {
        document.getElementById("clue").innerHTML = "Too high! Try again."
    } else if (randNum > inputVal) {
        document.getElementById("clue").innerHTML = "Congratulations! You won!"
        var names = ["guess", "title", "input", "btn"]
        names.forEach(name => {
            var elem = document.getElementById(name);
            elem.parentNode.removeChild(elem);
        })
        document.getElementById("reload").style.visibility = "visible";
    }
}
