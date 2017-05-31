# Number of dose units from beam 1;
var x_1 >= 0;
# Number of dose units from beam 2;
var x_2 >= 0;

# Minimize radiation dosage units to healthy anatomy (in kilorads);
minimize h_a_rad: 0.4*x_1 + 0.5*x_2;

# Exposure for critical tissues should not exceed 2.7 kilorads;
subject to critical_tissue_rad: 0.3*x_1 + 0.1*x_2 <= 2.7;
# Exposure for center of tumor should be at least 6 kilorads;
subject to tumor_center_rad: 0.6*x_1 + 0.4*x_2 >= 6;
# Exposure for tumor region should be exactly 6 kilorads;
subject to tumor_region_rad: 0.5*x_1 + 0.5*x_2 = 6;