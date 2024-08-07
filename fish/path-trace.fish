#!/usr/bin/env fish

function ___trace -S -d 'quick debugging tool'
    echo >&2
    set_color -d >&2
    status stack-trace &| string replace -r 'line (\d+) of file (.*)' '$2:$1' >&2
    echo >&2
    status stack-trace &| string match -r 'function \'[^\']*\'' >&2
    echo >&2
    set -l >&2
    set_color normal >&2
    echo >&2
end

function ___fail -S -d 'bail out'
    ___trace $argv
    echo -- "$argv" >&2
    exit 1
end

function ___math -d 'wrapper around builtin math'
    if not math $argv
        ___fail Invalid expression: $argv
    end
end

function ___arg_null -S -d 'ensure that no arguments exist'
    if set -q argv[1]
        ___fail Unexpected arguments: (string escape -- $argv)
    end
end

function ___arg_begin -S -d 'initialize arguments variable'
    set args $argv
    true
end

function ___arg_rest -S -d 'keep consuming arguments until none is left'
    set -l ___type

    while true
        if not set -q argv[1]
            ___fail Missing -- in ___arr_t: $argv
        else if test "$argv[1]" = --
            set -e argv[1]
            break
        else
            set -a ___type $argv[1]
            set -e argv[1]
        end
    end

    set -l ___len (___arg_join $argv len)
    set -a $___len 0

    while set -q args[1]
        $___type $argv items
        set $___len[1][-1] (___math $$___len[1][-1] + 1)
    end

    ___arg_end
end

function ___arg_end -S -d 'ensure that no more arguments are left'
    ___arg_null $argv $args
end

function ___arg_join -d 'get namespaced variable name'
    string join _ -- $argv
end

function ___arg_parse -S -d 'parse content of variable'
    set -l ___type

    while true
        if not set -q argv[1]
            ___fail Missing -- in ___arr_t: $argv
        else if test "$argv[1]" = --
            set -e argv[1]
            break
        else
            set -a ___type $argv[1]
            set -e argv[1]
        end
    end

    set -l ___base (___arg_join $argv)

    ___arg_begin $$___base

    set $___base

    for ___item in (set -ln | string match -re \^(string escape --style regex -- $___base)_)
        set $___item
    end

    $___type $argv
    ___arg_end
end

function ___str_t -S -d 'shift a string argument'
    if set -q args[1]
        for i in (seq (count $argv))
            set -a (___arg_join $argv[1..$i]) $args[1]
        end

        set -e args[1]
        true
    else
        ___fail Missing argument for (___arg_join $argv)
    end
end

function ___bool_t -S -d 'shift a boolean argument'
    ___str_t $argv
end

function ___f64_t -S -d 'shift a floating-point argument'
    ___str_t $argv

    set -l ___base (___arg_join $argv)

    switch $$___base[1][-1]
        case (___f64_nan) (___f64_inf) (___f64_neg_inf)
            ___fail Not a number: $___base
    end
end

function ___arr_t -S -d 'shift an array argument'
    set -l ___type

    while true
        if not set -q argv[1]
            ___fail Missing -- in ___arr_t: $argv
        else if test "$argv[1]" = --
            set -e argv[1]
            break
        else
            set -a ___type $argv[1]
            set -e argv[1]
        end
    end

    set -l ___len $argv len
    ___f64_t $___len
    set ___len (___arg_join $___len)

    for i in (seq $$___len[1][-1])
        $___type $argv items
    end
end

function ___opt_t -S -d 'shift an optional argument'
    set -l ___type

    while true
        if not set -q argv[1]
            ___fail Missing -- in ___opt_t: $argv
        else if test "$argv[1]" = --
            set -e argv[1]
            break
        else
            set -a ___type $argv[1]
            set -e argv[1]
        end
    end

    set -l ___some $argv some
    ___bool_t $___some
    set ___some (___arg_join $___some)

    if ___bool_if $$___some[1][-1]
        $___type $argv value
    end
end

function ___ivl_f64_t -S -d 'shift a interval floating-point argument'
    ___str_t $argv

    set -l ___base (___arg_join $argv)

    switch $$___base[1][-1]
        case (___f64_nan)
            ___fail Not a number: $___base
    end
end

function ___ivl_t -S -d 'shift an interval argument'
    ___ivl_f64_t $argv min
    ___ivl_f64_t $argv max
end

function ___vec_t -S -d 'shift a vector argument'
    ___f64_t $argv x
    ___f64_t $argv y
    ___f64_t $argv z
end

function ___ray_t -S -d 'shift a ray argument'
    ___vec_t $argv org
    ___vec_t $argv dir


    set -l ___base (___arg_join $argv)

    if ___vec_if_near_zero $$___base[1][-3..-1]
        ___fail Ray has a zero direction: $___base
    end
end

function ___mat_lambertian_t -S -d 'shift a lambertian material argument'
    ___vec_t $argv albedo
end

function ___mat_metal_t -S -d 'shift a metal material argument'
    ___vec_t $argv albedo
    ___f64_t $argv fuzz
end

function ___mat_dielectric_t -S -d 'shift a dielectric metal argument'
    ___f64_t $argv refraction_index
end

function ___mat_t -S -d 'shift a material argument'
    set -l ___type $argv type
    ___f64_t $___type
    set ___type (___arg_join $___type)

    switch $$___type[1][-1]
        case none

        case lambertian
            ___mat_lambertian_t $argv

        case metal
            ___mat_metal_t $argv

        case dielectric
            ___mat_dielectric_t $argv

        case \*
            ___fail Invalid material type $$___type[1][-1]
    end
end

function ___rec_t -S -d 'shift a hit record argument'
    ___f64_t $argv t
    ___vec_t $argv p
    ___mat_t $argv mat
    ___f64_t $argv front_face
    ___vec_t $argv normal
end

function ___opt_rec_t -S -d 'shift an optional hit record argument'
    ___opt_t ___rec_t -- $argv
end

function ___sct_t -S -d 'shift a scatter record argument'
    ___vec_t $argv att
    ___ray_t $argv sct
end

function ___opt_sct_t -S -d 'shift an scatter hit record argument'
    ___opt_t ___sct_t -- $argv
end

function ___hit_sphere_t -S -d 'shift a sphere hittable argument'
    ___vec_t $argv c
    ___f64_t $argv r
    ___mat_t $argv mat
end

function ___hit_list_t -S -d 'shift a hittable list argument'
    ___arr_t ___hit_t -- $argv objs
end

function ___hit_t -S -d 'shift a hittable argument'
    set -l ___type $argv type
    ___str_t $___type
    set ___type (___arg_join $___type)

    switch $$___type[1][-1]
        case none

        case sphere
            ___hit_sphere_t $argv

        case list
            if contains items $argv
                ___trace Nested list found: $argv
            end

            ___hit_list_t $argv

        case \*
            ___fail Invalid hittable type $$___type[1][-1]
    end
end

function ___cam_opt_t -S -d 'shift a camera options argument'
    ___f64_t $argv aspect_ratio
    ___f64_t $argv image_width
    ___f64_t $argv samples_per_pixel
    ___f64_t $argv max_depth
    ___f64_t $argv v_fov
    ___vec_t $argv look_from
    ___vec_t $argv look_at
    ___vec_t $argv vup
    ___f64_t $argv defocus_angle
    ___f64_t $argv focus_dist
end

function ___cam_t -S -d 'shift a camera argument'
    ___f64_t $argv image_width
    ___f64_t $argv image_height
    ___vec_t $argv center
    ___vec_t $argv pixel00_loc
    ___vec_t $argv pixel_delta_u
    ___vec_t $argv pixel_delta_v
    ___f64_t $argv defocus_angle
    ___vec_t $argv defocus_disk_u
    ___vec_t $argv defocus_disk_v
    ___f64_t $argv samples_per_pixel
    ___f64_t $argv pixel_samples_scale
    ___f64_t $argv max_depth
end

function ___value -d 'yield a value'
    string join \n -- $argv
    true
end

function ___f64_add -d 'add f64 values'
    ___math 0 +\($argv\)
end

function ___f64_sub -d 'subtract two f64 values'
    ___arg_begin $argv
    ___f64_t a
    ___f64_t b
    ___arg_end

    ___math $a - $b
end

function ___f64_mul -d 'multiply f64 values'
    ___math 1 \*\($argv\)
end

function ___f64_div -d 'divide two f64 values'
    ___arg_begin $argv
    ___f64_t a
    ___f64_t b
    ___arg_end

    if ___f64_if_eq $b 0
        ___fail Division by zero: $argv
    end

    ___math $a / $b
end

function ___f64_pow -d 'f64 to the power of f64'
    ___arg_begin $argv
    ___f64_t a
    ___f64_t b
    ___arg_end

    if ___f64_if_eq $a 0
        and ___f64_if_eq $b 0

        ___fail Zero to the power of zero: $argv
    end

    ___math $a \^ $b
end

function ___f64_sqrt -d 'square root of f64 value'
    ___arg_begin $argv
    ___f64_t value
    ___arg_end

    ___math sqrt $value
end

function ___f64_abs -d 'absolute value of f64 value'
    ___arg_begin $argv
    ___f64_t value
    ___arg_end

    ___math abs $value
end

function ___f64_min -d 'minimum of f64 values'
    ___math min\ $argv, (___f64_inf)
end

function ___f64_max -d 'maximum of f64 values'
    ___math max\ $argv, (___f64_neg_inf)
end

function ___f64_trunc -d 'truncation of f64 value'
    ___arg_begin $argv
    ___f64_t value
    ___arg_end

    ___math -s 0 $value
end

function ___f64_rad -d 'radian of f64 degree value'
    ___arg_begin $argv
    ___f64_t value
    ___arg_end

    ___math $value \* pi / 180
end

function ___f64_tan -d 'tangent of f64 radian value'
    ___arg_begin $argv
    ___f64_t value
    ___arg_end

    ___math tan $value
end

function ___f64_nan -d 'get nan'
    ___arg_null $argv

    ___value NaN
end

function ___f64_inf -d 'get positive infinity'
    ___arg_null $argv

    ___value 1e309
end

function ___f64_neg_inf -d 'get negitive infinity'
    ___arg_null $argv

    ___value -1e309
end

function ___bool_if -d 'evaluate boolean expressions to $status'
    ___arg_begin $argv
    ___bool_t expr
    ___arg_end

    switch $expr
        case 1
            true

        case 0
            false

        case \*
            ___fail Invalid boolean: $argv
    end
end

function ___f64_if_lt -d 'check if f64 is less than other'
    ___arg_begin $argv
    ___f64_t a
    ___f64_t b
    ___arg_end

    test $a -lt $b
end

function ___f64_if_le -d 'check if f64 is less than or equal to other'
    ___arg_begin $argv
    ___f64_t a
    ___f64_t b
    ___arg_end

    test $a -le $b
end

function ___f64_if_eq -d 'check if two f64 are equal'
    ___arg_begin $argv
    ___f64_t a
    ___f64_t b
    ___arg_end

    test $a -eq $b
end

function ___f64_if_ne -d 'check if two f64 are not equal'
    ___arg_begin $argv
    ___f64_t a
    ___f64_t b
    ___arg_end

    test $a -ne $b
end

function ___f64_if_ge -d 'check if f64 is greater than or equal to other'
    ___arg_begin $argv
    ___f64_t a
    ___f64_t b
    ___arg_end

    test $a -ge $b
end

function ___f64_if_gt -d 'check if f64 is greater than other'
    ___arg_begin $argv
    ___f64_t a
    ___f64_t b
    ___arg_end

    test $a -gt $b
end

function ___arr_get -d 'get the ith item of array (one-indexed)'
    set type

    while true
        if not set -q argv[1]
            ___fail Missing -- in ___arr_get: $argv
        else if test "$argv[1]" = --
            set -e argv[1]
            break
        else
            set -a type $argv[1]
            set -e argv[1]
        end
    end

    ___arg_begin (___f64_sub $argv[1] 1) $argv[2..]
    ___arr_t $type -- pre
    $type value
    ___arg_rest $type -- post

    ___value $value
end

function ___f64_random
    ___arg_null $argv

    ___f64_div (random 0 32768) 32768
end

function ___f64_lerp
    ___arg_begin $argv
    ___f64_t t
    ___f64_t a
    ___f64_t b
    ___arg_end

    ___f64_add (___f64_mul $a (___f64_sub 1 $t)) (___f64_mul $b $t)
end

function ___ppm_begin
    ___arg_begin $argv
    ___f64_t width
    ___f64_t height
    ___arg_end

    echo P3
    echo "$width $height"
    echo 255
end

function ___ppm_pixel
    ___arg_begin $argv
    ___vec_t color
    ___arg_end

    echo "$color_x $color_y $color_z"
end

function ___ppm_end
    ___arg_null $argv
end

function ___ivl_empty
    ___arg_null $argv

    ___f64_inf
    ___f64_neg_inf
end

function ___ivl_universe
    ___arg_null $argv

    ___f64_neg_inf
    ___f64_inf
end

function ___ivl_if_contains
    ___arg_begin $argv
    ___ivl_t self
    ___f64_t x
    ___arg_end

    switch $self_min
        case (___f64_inf)
            false
            return

        case (___f64_neg_inf)

        case \*
            ___f64_if_le $self_min $x || return
    end

    switch $self_max
        case (___f64_inf)

        case (___f64_neg_inf)
            false
            return

        case \*
            ___f64_if_le $x $self_max || return
    end
end

function ___ivl_if_surrounds
    ___arg_begin $argv
    ___ivl_t self
    ___f64_t x
    ___arg_end

    switch $self_min
        case (___f64_inf)
            false
            return

        case (___f64_neg_inf)

        case \*
            ___f64_if_lt $self_min $x || return
    end

    switch $self_max
        case (___f64_inf)

        case (___f64_neg_inf)
            false
            return

        case \*
            ___f64_if_lt $x $self_max || return
    end
end

function ___ivl_clamp
    ___arg_begin $argv
    ___ivl_t self
    ___f64_t x
    ___arg_end

    switch $self_min
        case (___f64_inf)
            ___fail Empty interval given: $argv

        case (___f64_neg_inf)

        case \*
            ___f64_if_lt $x $self_min && set x $self_min
    end

    switch $self_max
        case (___f64_inf)

        case (___f64_neg_inf)
            ___fail Empty interval given: $argv

        case \*
            ___f64_if_gt $x $self_max && set x $self_max
    end

    ___value $x
end

function ___ivl_random
    ___arg_begin $argv
    ___ivl_t self
    ___arg_end

    switch $self_min
        case (___f64_inf)
            ___fail Empty interval given: $argv

        case (___f64_neg_inf)
            ___fail Expected a bounded interval: $argv
    end

    switch $self_max
        case (___f64_inf)
            ___fail Expected a bounded interval: $argv

        case (___f64_neg_inf)
            ___fail Empty interval given: $argv

    end

    ___f64_lerp (___f64_random) $self
end

function ___vec_f64
    ___arg_begin $argv
    ___f64_t value
    ___arg_end

    ___value $value $value $value
end

function ___vec_dot
    ___arg_begin $argv
    ___vec_t self
    ___vec_t rhs
    ___arg_end

    ___f64_add \
        (___f64_mul $self_x $rhs_x) \
        (___f64_mul $self_y $rhs_y) \
        (___f64_mul $self_z $rhs_z)
end

function ___vec_cross
    ___arg_begin $argv
    ___vec_t self
    ___vec_t rhs
    ___arg_end

    ___f64_sub (___f64_mul $self_y $rhs_z) (___f64_mul $self_z $rhs_y)
    ___f64_sub (___f64_mul $self_z $rhs_x) (___f64_mul $self_x $rhs_z)
    ___f64_sub (___f64_mul $self_x $rhs_y) (___f64_mul $self_y $rhs_x)
end

function ___vec_squared_abs
    ___arg_begin $argv
    ___vec_t self
    ___arg_end

    ___vec_dot $self $self
end

function ___vec_abs
    ___arg_begin $argv
    ___vec_t self
    ___arg_end

    ___f64_sqrt (___vec_squared_abs $self)
end

function ___vec_add -d 'add vector values'
    ___arg_begin $argv
    ___arg_rest ___vec_t -- rest

    ___f64_add $rest_items_x
    ___f64_add $rest_items_y
    ___f64_add $rest_items_z
end

function ___vec_sub -d 'subtract two vector values'
    ___arg_begin $argv
    ___vec_t a
    ___vec_t b
    ___arg_end

    ___f64_sub $a_x $b_x
    ___f64_sub $a_y $b_y
    ___f64_sub $a_z $b_z
end

function ___vec_mul -d 'multiply vector values'
    ___arg_begin $argv
    ___arg_rest ___vec_t -- rest

    ___f64_mul $rest_items_x
    ___f64_mul $rest_items_y
    ___f64_mul $rest_items_z
end

function ___vec_div -d 'divide two vector values'
    ___arg_begin $argv
    ___vec_t a
    ___vec_t b
    ___arg_end

    ___f64_div $a_x $b_x
    ___f64_div $a_y $b_y
    ___f64_div $a_z $b_z
end

function ___vec_normalize
    ___arg_begin $argv
    ___vec_t self
    ___arg_end

    set abs (___vec_abs $self)

    if ___f64_if_eq $abs 0
        ___fail Normalizing the zero vector: $argv
    end

    ___vec_div $self (___vec_f64 $abs)
end

function ___vec_random
    ___arg_null $argv

    ___f64_random
    ___f64_random
    ___f64_random
end

function ___vec_random_between
    ___arg_begin $argv
    ___ivl_t ivl
    ___arg_end

    ___ivl_random $ivl
    ___ivl_random $ivl
    ___ivl_random $ivl
end

function ___vec_random_in_unit_sphere
    ___arg_null $argv

    while true
        set p (___vec_random_between -1 1)

        if ___f64_if_lt (___vec_squared_abs $p) 1
            ___value $p
            return
        end
    end
end

function ___vec_random_in_unit_disk
    ___arg_null $argv

    while true
        set p \
            (___ivl_random -1 1) \
            (___ivl_random -1 1) \
            0

        if ___f64_if_lt (___vec_squared_abs $p) 1
            ___value $p
            return
        end
    end
end

function ___vec_random_normalized
    ___arg_null $argv

    ___vec_normalize (___vec_random_in_unit_sphere)
end

function ___vec_random_on_hemisphere
    ___arg_begin $argv
    ___vec_t normal
    ___arg_end

    set p (___vec_random_normalized)

    if ___f64_if_gt (___vec_dot $p $normal) 0
        ___value $p
    else
        ___vec_sub 0 0 0 $p
    end
end

function ___vec_if_near_zero
    ___arg_begin $argv
    ___vec_t self
    ___arg_end

    set s 1e-8
    ___f64_if_lt (___f64_abs $self_x) $s
    and ___f64_if_lt (___f64_abs $self_y) $s
    and ___f64_if_lt (___f64_abs $self_z) $s
end

function ___vec_reflect
    ___arg_begin $argv
    ___vec_t self
    ___vec_t normal
    ___arg_end

    ___vec_sub $self (___vec_mul $normal (___vec_f64 (___f64_mul 2 (___vec_dot $self $normal))))
end

function ___vec_refract
    ___arg_begin $argv
    ___vec_t self
    ___vec_t normal
    ___f64_t eta_i_over_eta_t
    ___arg_end

    set cosq (___f64_min (___vec_dot (___vec_sub 0 0 0 $self) $normal) 1)

    set r_out_perp (___vec_mul (___vec_add $self (___vec_mul $normal (___vec_f64 $cosq))) (___vec_f64 $eta_i_over_eta_t))

    set r_out_par (___vec_mul $normal (___vec_f64 (___f64_sub 0 (___f64_sqrt (___f64_abs (___f64_sub 1 (___vec_squared_abs $r_out_perp)))))))

    ___vec_add $r_out_perp $r_out_par
end

function ___vec_to_ppm
    ___arg_begin $argv
    ___vec_t self
    ___arg_end

    set intensity 0 0.999

    ___f64_trunc (___f64_mul 256 (___f64_sqrt (___ivl_clamp $intensity $self_x)))
    ___f64_trunc (___f64_mul 256 (___f64_sqrt (___ivl_clamp $intensity $self_y)))
    ___f64_trunc (___f64_mul 256 (___f64_sqrt (___ivl_clamp $intensity $self_z)))
end

function ___vec_lerp
    ___arg_begin $argv
    ___f64_t t
    ___vec_t a
    ___vec_t b
    ___arg_end

    ___vec_add (___vec_mul $a (___vec_f64 (___f64_sub 1 $t))) (___vec_mul $b (___vec_f64 $t))
end

function ___ray_at
    ___arg_begin $argv
    ___ray_t self
    ___f64_t t
    ___arg_end

    ___vec_add $self_org (___vec_mul $self_dir (___vec_f64 $t))
end

function ___mat_scatter
    ___arg_begin $argv
    ___mat_t self
    ___ray_t r_in
    ___rec_t rec
    ___arg_end

    switch $self_type
        case none
            ___value 0

        case lambertian
            set dir (___vec_add $rec_normal (___vec_random_normalized))

            if ___vec_if_near_zero $dir
                set dir $rec_normal
            end

            set att $self_albedo
            set sct $rec_p $dir

            ___value 1 $att $sct

        case metal
            set refl (___vec_add (___vec_reflect $r_in_dir $rec_normal) (___vec_mul (___vec_random_normalized) (___vec_f64 $self_fuzz)))
            set att $self_albedo
            set sct $rec_p $refl

            if ___f64_if_gt (___vec_dot $refl $rec_normal) 0
                ___value 1 $att $sct
            else
                ___value 0
            end

        case dielectric
            if ___bool_if $rec_front_face
                set ri (___f64_div 1 $self_refraction_index)
            else
                set ri $self_refraction_index
            end

            set unit_dir (___vec_normalize $r_in_dir)
            set cosq (___f64_min (___vec_dot (___vec_sub 0 0 0 $unit_dir) $rec_normal) 1)
            set sinq (___f64_sqrt (___f64_sub 1 (___f64_mul $cosq $cosq)))

            if ___f64_if_gt (___f64_mul $ri $sinq) 1
                set should_reflect 1
            else
                set r0 (___f64_div (___f64_sub 1 $ri) (___f64_add 1 $ri))
                set r0 (___f64_mul $r0 $r0)
                set r0 (___f64_add $r0 (___f64_mul (___f64_sub 1 $r0) (___f64_pow (___f64_sub 1 $cosq) 5)))

                if ___f64_if_gt $r0 (___f64_random)
                    set should_reflect 1
                else
                    set should_reflect 0
                end
            end

            if ___bool_if $should_reflect
                set dir (___vec_reflect $unit_dir $rec_normal)
            else
                set dir (___vec_refract $unit_dir $rec_normal $ri)
            end

            set att 1 1 1
            set sct $rec_p $dir
            ___value 1 $att $sct

        case \*
            ___fail Invalid material type $self_type
    end
end

function ___rec_set_face_normal
    ___arg_begin $argv
    ___rec_t self
    ___ray_t r
    ___vec_t outward_normal
    ___arg_end

    if ___f64_if_lt (___vec_dot $r_dir $outward_normal) 0
        set self_front_face 1
    else
        set self_front_face 0
    end

    if ___bool_if $self_front_face
        set self_normal $outward_normal
    else
        set self_normal (___vec_sub 0 0 0 $outward_normal)
    end

    ___value $self_t $self_p $self_mat $self_front_face $self_normal
end

function ___rec_new
    ___arg_begin $argv
    ___f64_t t
    ___vec_t p
    ___mat_t mat
    ___ray_t r
    ___vec_t outward_normal
    ___arg_end

    ___rec_set_face_normal $t $p $mat 0 0 0 0 $r $outward_normal
end

function ___hit
    ___arg_begin $argv
    ___hit_t self
    ___ray_t r
    ___ivl_t ray_t
    ___arg_end

    switch $self_type
        case sphere
            set oc (___vec_sub $self_c $r_org)
            set a (___vec_squared_abs $r_dir)
            set h (___vec_dot $r_dir $oc)
            set c (___f64_sub (___vec_squared_abs $oc) (___f64_mul $self_r $self_r))
            set d (___f64_sub (___f64_mul $h $h) (___f64_mul $a $c))

            if ___f64_if_lt $d 0
                ___value 0
                return
            end

            set sqrt_d (___f64_sqrt $d)
            set root (___f64_div (___f64_sub $h $sqrt_d) $a)

            if not ___ivl_if_surrounds $ray_t $root
                set root (___f64_div (___f64_add $h $sqrt_d) $a)

                if not ___ivl_if_surrounds $ray_t $root
                    ___value 0
                    return
                end
            end

            set t $root
            set p (___ray_at $r $t)
            set outward_normal (___vec_div (___vec_sub $p $self_c) (___vec_f64 $self_r))

            ___value 1
            ___rec_new $t $p $self_mat $r $outward_normal

        case list
            set closest_rec 0
            set closest_so_far $ray_t_max

            for i in (seq $self_objs_len)
                set obj (___arr_get ___hit_t -- $i $self_objs_items)
                set rec (___hit $obj $r $ray_t_min $closest_so_far)

                ___arg_parse ___opt_rec_t -- rec

                if ___bool_if $rec_some
                    set closest_so_far $rec_value_t
                    set closest_rec $rec
                end
            end

            ___value $closest_rec

        case \*
            ___fail Invalid hittable type $self_type
    end
end

# camera

function ___cam_new
    ___arg_begin $argv
    ___cam_opt_t o
    ___arg_end

    set image_width $o_image_width
    set image_height (___f64_max (___f64_trunc (___f64_div $image_width $o_aspect_ratio)) 1)

    # Camera

    set tetha (___f64_rad $o_v_fov)
    set h (___f64_tan (___f64_div $tetha 2))
    set viewport_height (___f64_mul 2 $h $o_focus_dist)
    set viewport_width (___f64_div (___f64_mul $viewport_height $image_width) $image_height)
    set center $o_look_from

    set w (___vec_normalize (___vec_sub $o_look_from $o_look_at))
    set u (___vec_normalize (___vec_cross $o_vup $w))
    set v (___vec_cross $w $u)

    set viewport_u (___vec_mul $u (___vec_f64 $viewport_width))
    set viewport_v (___vec_mul $v (___vec_f64 (___f64_sub 0 $viewport_height)))

    set pixel_delta_u (___vec_div $viewport_u (___vec_f64 $image_width))
    set pixel_delta_v (___vec_div $viewport_v (___vec_f64 $image_height))

    set viewport_upper_left (___vec_sub (___vec_sub $center (___vec_mul $w (___vec_f64 $o_focus_dist))) (___vec_div (___vec_add $viewport_u $viewport_v) 2 2 2))

    set pixel00_loc (___vec_add $viewport_upper_left (___vec_div (___vec_add $pixel_delta_u $pixel_delta_v) 2 2 2))

    # Anti-aliasing

    set samples_per_pixel $o_samples_per_pixel
    set pixel_samples_scale (___f64_div 1 $samples_per_pixel)

    # Diffuse

    set max_depth $o_max_depth

    # Defocus Blur

    set defocus_angle $o_defocus_angle
    set defocus_radius (___f64_mul $o_focus_dist (___f64_tan (___f64_rad (___f64_div $defocus_angle 2))))

    set defocus_disk_u (___vec_mul $u (___vec_f64 $defocus_radius))
    set defocus_disk_v (___vec_mul $v (___vec_f64 $defocus_radius))

    ___value \
        $image_width \
        $image_height \
        $center \
        $pixel00_loc \
        $pixel_delta_u \
        $pixel_delta_v \
        $defocus_angle \
        $defocus_disk_u \
        $defocus_disk_v \
        $samples_per_pixel \
        $pixel_samples_scale \
        $max_depth
end

function ___cam_sample_square
    ___arg_null $argv

    ___value \
        (___f64_sub (___f64_random) 0.5) \
        (___f64_sub (___f64_random) 0.5) \
        0
end

function ___cam_sample_defocus_disk
    ___arg_begin $argv
    ___cam_t self
    ___arg_end

    set p (___vec_random_in_unit_disk)
    ___arg_parse ___vec_t -- p

    ___vec_add $self_center \
        (___vec_mul $self_defocus_disk_u (___vec_f64 $p_x)) \
        (___vec_mul $self_defocus_disk_v (___vec_f64 $p_y))
end

function ___cam_get_ray
    ___arg_begin $argv
    ___cam_t self
    ___f64_t x
    ___f64_t y
    ___arg_end

    set offset (___cam_sample_square)
    ___arg_parse ___vec_t -- offset

    set pixel_sample (
        ___vec_add $self_pixel00_loc \
            (___vec_mul $self_pixel_delta_u (___vec_f64 (___f64_add $x $offset_x))) \
            (___vec_mul $self_pixel_delta_v (___vec_f64 (___f64_add $y $offset_y)))
    )

    if ___f64_if_le $self_defocus_angle 0
        set origin $self_center
    else
        set origin (___cam_sample_defocus_disk $self)
    end

    ___value $origin (___vec_sub $pixel_sample $origin)
end

function ___cam_ray_color
    ___arg_begin $argv
    ___ray_t r
    ___f64_t depth
    ___hit_t world
    ___arg_end

    if ___f64_if_eq $depth 0
        ___value 0 0 0
        return
    end

    set rec (___hit $world $r 0.001 (___f64_inf))
    ___arg_parse ___opt_rec_t -- rec

    if ___bool_if $rec_some
        set sct (___mat_scatter $rec_value_mat $r $rec_value)
        ___arg_parse ___opt_sct_t -- sct

        if ___bool_if $sct_some
            ___vec_mul (___cam_ray_color $sct_value_sct (___f64_sub $depth 1) $world) $sct_value_att
            return
        end

        ___value 0 0 0
        return
    end

    set udir (___vec_normalize $r_dir)
    ___arg_parse ___vec_t -- udir

    set a (___f64_div (___f64_add $udir_y 1) 2)
    ___vec_lerp $a 1 1 1 0.5 0.7 1
end

function ___cam_render_pixel
    ___arg_begin $argv
    ___cam_t self
    ___hit_t world
    ___f64_t x
    ___f64_t y
    ___arg_end

    set color 0 0 0

    for _sample in (seq $self_samples_per_pixel)
        set ray (___cam_get_ray $self $x $y)

        set color (___vec_add $color (___cam_ray_color $ray $self_max_depth $world))
    end

    ___vec_to_ppm (___vec_mul $color (___vec_f64 $self_pixel_samples_scale))
end

function ___cam_render
    ___arg_begin $argv
    ___cam_t self
    ___hit_t world
    ___arg_end

    ___ppm_begin $self_image_width $self_image_height

    for y in (seq 0 (___f64_sub $self_image_height 1))
        for x in (seq 0 (___f64_sub $self_image_width 1))
            echo -n "y=$y x=$x rendering" >&2
            ___ppm_pixel (___cam_render_pixel $self $world $x $y)
            echo \r"y=$y x=$x rendered " >&2
        end
    end

    ___ppm_end
end

function ___cam_render_parallel
    ___arg_begin $argv
    ___cam_t self
    ___hit_t world
    ___arg_end

    if not set nproc (nproc --all --ignore=1)
        or ___f64_if_lt $nproc 2

        ___cam_render $argv
        return
    end

    set file (status filename)
    set temp (mktemp -d --suffix .path-trace.fish)

    for y in (seq 0 (___f64_sub $self_image_height 1))
        for x in (seq 0 (___f64_sub $self_image_width 1))
            while ___f64_if_ge (jobs | count) $nproc
                echo -n \r"y=$y x=$x queued  " >&2
                wait -n
            end

            echo -n \r"y=$y x=$x spawning" >&2
            fish $file thread $self $world $x $y >$temp/$y-$x &
            echo -n \r"y=$y x=$x spawned " >&2
        end
    end

    echo >&2
    wait

    ___ppm_begin $self_image_width $self_image_height
    cat $temp/(seq 0 (___f64_sub $self_image_height 1))-(seq 0 (___f64_sub $self_image_width 1))
    ___ppm_end

    rm -r $temp
end

function ___cam_render_thread
    ___arg_begin $argv
    ___cam_t self
    ___hit_t world
    ___f64_t x
    ___f64_t y
    ___arg_end

    ___ppm_pixel (___cam_render_pixel $self $world $x $y)
end

function ___main
    set material_ground lambertian 0.8 0.8 0
    set material_center lambertian 0.1 0.2 0.5
    set material_left dielectric 1.5
    set material_bubble dielectric 0.666666667
    set material_right metal 0.8 0.6 0.2 1

    set world list 5 \
        sphere 0 -100.5 -1 100 $material_ground \
        sphere 0 0 -1.2 0.5 $material_center \
        sphere -1 0 -1 0.5 $material_left \
        sphere -1 0 -1 0.4 $material_bubble \
        sphere 1 0 -1 0.5 $material_right

    set camera (
        ___cam_new \
            1.777777778 \
            40 \
            1 \
            50 \
            20 \
            -2 2 1 \
            0 0 -1 \
            0 1 0 \
            10 \
            3.4
    )

    ___value $camera $world
end

switch "$argv[1]"
    case sequence
        ___cam_render (___main)

    case parallel
        ___cam_render_parallel (___main)

    case thread
        ___cam_render_thread $argv[2..]
end
