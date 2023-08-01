mod ft_data;

fn main() {
    // TODO: use ft_data
    // TODO: use ft_fields

    // kari
    let t_max = 100;


    // Run the entire problem once to make sure all data is touched.
    // This reduces variable startup costs, which is important for such a
    // short benchmark. The other NPB 2 implementations are similar.
    for i in 1..=t_max {
        // call timer_clear(i)
    }

    // call alloc_space
    let ft_data = setup_class_c();
    setup(ft_data);
    // call setup()
    // call init_ui(u0, u1, twiddle, dims(1), dims(2), dims(3))
    //  call compute_indexmap(twiddle, dims(1), dims(2), dims(3))
    //  call compute_initial_conditions(u1, dims(1), dims(2), dims(3))
    //  call fft_init (dims(1))
    //  call fft(1, u1, u0)

    // Start over from the beginning. Note that all operations must
    // be timed, in contrast to other benchmarks.

    for i in 1..=t_max {
        // call timer_clear(i)
    }

    // call timer_start(T_total)
    // if (timers_enabled) call timer_start(T_setup)

    // call compute_indexmap(twiddle, dims(1), dims(2), dims(3))
//     call compute_initial_conditions(u1, dims(1), dims(2), dims(3))

//     call fft_init (dims(1))

//     if (timers_enabled) call timer_stop(T_setup)
//     if (timers_enabled) call timer_start(T_fft)
//     call fft(1, u1, u0)
//     if (timers_enabled) call timer_stop(T_fft)

//     do iter = 1, niter
//        if (timers_enabled) call timer_start(T_evolve)
//        call evolve(u0, u1, twiddle, dims(1), dims(2), dims(3))
//        if (timers_enabled) call timer_stop(T_evolve)
//        if (timers_enabled) call timer_start(T_fft)
// !         call fft(-1, u1, u2)
//        call fft(-1, u1, u1)
//        if (timers_enabled) call timer_stop(T_fft)
//        if (timers_enabled) call timer_start(T_checksum)
// !         call checksum(iter, u2, dims(1), dims(2), dims(3))
//        call checksum(iter, u1, dims(1), dims(2), dims(3))
//        if (timers_enabled) call timer_stop(T_checksum)
//     end do

// call verify(nx, ny, nz, niter, verified, class)

// call timer_stop(t_total)
// total_time = timer_read(t_total)

// if( total_time .ne. 0. ) then
//    mflops = 1.0d-6*ntotal_f *  &
// &             (14.8157+7.19641*log(ntotal_f)  &
// &          +  (5.23518+7.21113*log(ntotal_f))*niter)  &
// &                 /total_time
// else
//    mflops = 0.0
// endif
// call print_results('FT', class, nx, ny, nz, niter,  &
// &  total_time, mflops, '          floating point', verified,  &
// &  npbversion, compiletime, cs1, cs2, cs3, cs4, cs5, cs6, cs7)
// if (timers_enabled) call print_timers()
}

fn setup_class_c() -> ft_data::FtData {
    ft_data::FtData {
        nx: 512,
        ny: 512,
        nz: 512,
        nxp: 512 + 1,
        ntotalp: (512 + 1) * 512 + 512,
        ntotal_f: 512.0 * 512.0 * 512.0,
        fftblock: 32,
        fftblockpad: 32 + 2,
        dims: [ 512, 512, 512 ],
        timers_enabled: true,
        timer_read: 1.0,    // kari
        ilog2: 10,  // kari
        randlc: 0.0,    // kari
        debug: false,
        debugsynch: false,
        niter: 20,
    }
}

fn setup(ft_data: ft_data::FtData) {
    // kari
    let threads = 1;

    println!(" NAS Parallel Benchmarks (NPB3.4-OMP) - FT Benchmark");
    println!(" Size                : {0: >4}x{1: >4}x{2: >4}", ft_data.nx, ft_data.ny, ft_data.nz);
    println!(" Iterations                  :{0: >7}", ft_data.niter);
    println!(" Number of available threads :{0: >7}", threads);

    /*---------------------------------------------------------------------
     * Set up info for blocking of ffts and transposes.  This improves
     * performance on cache-based systems. Blocking involves
     * working on a chunk of the problem at a time, taking chunks
     * along the first, second, or third dimension.
     *
     * - In cffts1 blocking is on 2nd dimension (with fft on 1st dim)
     * - In cffts2/3 blocking is on 1st dimension (with fft on 2nd and 3rd dims)
     *
     * Since 1st dim is always in processor, we'll assume it's long enough
     * (default blocking factor is 16 so min size for 1st dim is 16)
     * The only case we have to worry about is cffts1 in a 2d decomposition.
     * so the blocking factor should not be larger than the 2nd dimension.
     ---------------------------------------------------------------------*/
    //  fftblock = fftblock_default
    //  fftblockpad = fftblockpad_default

    //  if (fftblock .ne. fftblock_default) fftblockpad = fftblock+3
}