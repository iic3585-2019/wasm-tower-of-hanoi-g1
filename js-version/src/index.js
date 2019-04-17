import {fromEvent} from 'rxjs';
import timedHanoi from './hanoi';
import './styles/index.scss';


fromEvent(window, 'load').subscribe(() => {
  const nDisksInput = document.getElementById('n-disks-input');
  const timeResultText = document.getElementById('seconds');
  const nDisksText = document.getElementById('n-disks');

  const calculate = () => {
    const number = parseInt(nDisksInput.value);
    const time = timedHanoi(number);
    nDisksText.innerText = `${number}`;
    timeResultText.innerText = `${time}`;
  };

  fromEvent(document.getElementById('solve'), 'click').subscribe(calculate);
});

if (module.hot) {
  module.hot.accept();
}
