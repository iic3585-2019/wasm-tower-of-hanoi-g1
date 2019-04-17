import("../crate/pkg").then(module => {
  module.run();

  const solveButton = document.getElementById("solve");
  const nDisksInput = document.getElementById("n-disks-input")
  const timeResultText = document.getElementById("seconds");
  const nDisksText = document.getElementById("n-disks");

  const calculate = async () => {
    const number = parseInt(nDisksInput.value);
    const time = await module.timed_hanoi(number);
    nDisksText.innerText = `${number}`;
    timeResultText.innerText = `${time}`;
  }

  solveButton.addEventListener("click", calculate);

});
