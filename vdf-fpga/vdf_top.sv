module vdf_top #(
  parameter WIDTH = 256
)(
  input  logic              clk,
  input  logic              rst,
  input  logic [WIDTH-1:0]   x,
  input  logic               start,
  output logic [WIDTH-1:0]   y,
  output logic               done
);
  localparam ITER = 2_000_000; // calibrated for 2 s @ 100 MHz
  logic [31:0] ctr;
  always_ff @(posedge clk) begin
    if (rst) begin
      y <= x;
      ctr <= 0;
      done <= 0;
    end else if (start & !done) begin
      y <= (y * y) % 115792089237316195423570985008687907853269984665640564039457584007913129640747;
      ctr <= ctr + 1;
      if (ctr == ITER-1) done <= 1;
    end
  end
endmodule
