# Radiation Example;

var x_1;
var x_2;


minimize healthy_anatomy_exposure: 0.4*x_1 + 0.5*x_2
subject to critical_tissues: 0.3*x_1 + 0.1*x_2 <= 2.7;
subject to tumor_region: 0.5*x_1 + 0.5*x_2 >= 6;
subject to tumor_center: 0.6*x_1 + 0.4*x_2 == 6;