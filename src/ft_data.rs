pub struct FtData {
    // todo: include npbparams.h
    
    pub nx: i32,
    pub ny: i32,
    pub nz: i32,

    // nxp = nx + 1
    pub nxp: i32,
    // ntotalp = nxp * ny * nz
    pub ntotalp: i32,
    // ntotal_f = (double) nx * ny * nz
    pub ntotal_f: f64,

    // todo: include blk_par.h
    pub fftblock: i32,
    pub fftblockpad: i32,

    pub dims: [i32; 3],

    pub timers_enabled: bool,
    pub timer_read: f64,
    pub ilog2: i32,
    pub randlc: f64,

    // other stuff
    pub debug: bool,
    pub debugsynch: bool,

    // length: nxp
    // ビルド時にサイズが決まらないため除外
    //u: [f64],
    // todo: double complex sums(0:niter_default)
    pub niter: i32,
}

impl FtData {
    const T_total: i32 = 1;
    const T_setup: i32 = 2;
    const T_fft: i32 = 3;
    const T_evolve: i32 = 4;
    const T_checksum: i32 = 5;
    const T_fftx: i32 = 6;
    const T_ffty: i32 = 7;
    const T_fftz: i32 = 8;
    const T_max: i32 = 8;

    const seed: f64 = 314159265.0;
    const a: f64 = 1220703125.0;
    const pi: f64 = 3.141592653589793238;
    const alpha: f64 = 0.00001;
}