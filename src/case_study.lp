# This is a problem;

var television;
var newspaper;
var radio;

maximize objective: 100000.*television + 40000.*newspaper + 18000.*radio;

subject to constraint_1: 2000.*television + 600.*newspaper + 300.*radio <= 18200.;
subject to constraint_2: newspaper <= 10.;
subject to constraint_3: -1.*television + newspaper + radio <= 0.;
subject to constraint_4: -9.*television + newspaper + radio <= 0.;

# Nothing more to see here;