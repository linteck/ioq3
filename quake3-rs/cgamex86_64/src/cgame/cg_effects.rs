pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::GENDER_FEMALE;
pub use crate::bg_public_h::GENDER_MALE;
pub use crate::bg_public_h::GENDER_NEUTER;
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cgMedia_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::cgs_t;
pub use crate::cg_local_h::clientInfo_t;
pub use crate::cg_local_h::footstep_t;
pub use crate::cg_local_h::leBounceSoundType_t;
pub use crate::cg_local_h::leMarkType_t;
pub use crate::cg_local_h::leType_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::localEntity_s;
pub use crate::cg_local_h::localEntity_t;
pub use crate::cg_local_h::playerEntity_t;
pub use crate::cg_local_h::score_t;
pub use crate::cg_local_h::FOOTSTEP_BOOT;
pub use crate::cg_local_h::FOOTSTEP_ENERGY;
pub use crate::cg_local_h::FOOTSTEP_FLESH;
pub use crate::cg_local_h::FOOTSTEP_MECH;
pub use crate::cg_local_h::FOOTSTEP_METAL;
pub use crate::cg_local_h::FOOTSTEP_NORMAL;
pub use crate::cg_local_h::FOOTSTEP_SPLASH;
pub use crate::cg_local_h::FOOTSTEP_TOTAL;
pub use crate::cg_local_h::LEBS_BLOOD;
pub use crate::cg_local_h::LEBS_BRASS;
pub use crate::cg_local_h::LEBS_NONE;
pub use crate::cg_local_h::LEF_PUFF_DONT_SCALE;
pub use crate::cg_local_h::LEF_SOUND1;
pub use crate::cg_local_h::LEF_SOUND2;
pub use crate::cg_local_h::LEF_TUMBLE;
pub use crate::cg_local_h::LEMT_BLOOD;
pub use crate::cg_local_h::LEMT_BURN;
pub use crate::cg_local_h::LEMT_NONE;
pub use crate::cg_local_h::LE_EXPLOSION;
pub use crate::cg_local_h::LE_FADE_RGB;
pub use crate::cg_local_h::LE_FALL_SCALE_FADE;
pub use crate::cg_local_h::LE_FRAGMENT;
pub use crate::cg_local_h::LE_MARK;
pub use crate::cg_local_h::LE_MOVE_SCALE_FADE;
pub use crate::cg_local_h::LE_SCALE_FADE;
pub use crate::cg_local_h::LE_SCOREPLUM;
pub use crate::cg_local_h::LE_SPRITE_EXPLOSION;
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::cgame::cg_localents::CG_AllocLocalEntity;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_blood;
pub use crate::src::cgame::cg_main::cg_gibs;
pub use crate::src::cgame::cg_main::cg_noProjectileTrail;
pub use crate::src::cgame::cg_main::cg_scorePlum;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_Error;
pub use crate::src::qcommon::q_math::axisDefault;
pub use crate::src::qcommon::q_math::AnglesToAxis;
pub use crate::src::qcommon::q_math::AxisClear;
pub use crate::src::qcommon::q_math::AxisCopy;
pub use crate::src::qcommon::q_math::Q_random;
pub use crate::src::qcommon::q_math::RotateAroundDirection;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::RT_BEAM;
pub use crate::tr_types_h::RT_LIGHTNING;
pub use crate::tr_types_h::RT_MAX_REF_ENTITY_TYPE;
pub use crate::tr_types_h::RT_MODEL;
pub use crate::tr_types_h::RT_POLY;
pub use crate::tr_types_h::RT_PORTALSURFACE;
pub use crate::tr_types_h::RT_RAIL_CORE;
pub use crate::tr_types_h::RT_RAIL_RINGS;
pub use crate::tr_types_h::RT_SPRITE;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
//
// cg_effects.c -- these functions generate localentities, usually as a result
// of event processing
/*
==================
CG_BubbleTrail

Bullets shot underwater
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_BubbleTrail(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut spacing: f32,
) {
    let mut move_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut len: f32 = 0.;
    let mut i: i32 = 0;
    if crate::src::cgame::cg_main::cg_noProjectileTrail.integer != 0 {
        return;
    }
    move_0[0] = *start.offset(0);
    move_0[1] = *start.offset(1);
    move_0[2] = *start.offset(2);
    vec[0] = *end.offset(0) - *start.offset(0);
    vec[1] = *end.offset(1) - *start.offset(1);
    vec[2] = *end.offset(2) - *start.offset(2);
    len = crate::src::qcommon::q_math::VectorNormalize(vec.as_mut_ptr());
    // advance a random amount first
    i = crate::stdlib::rand() % spacing as i32;
    move_0[0] = move_0[0] + vec[0] * i as f32;
    move_0[1] = move_0[1] + vec[1] * i as f32;
    move_0[2] = move_0[2] + vec[2] * i as f32;
    vec[0] = vec[0] * spacing;
    vec[1] = vec[1] * spacing;
    vec[2] = vec[2] * spacing;
    while (i as f32) < len {
        let mut le: *mut crate::cg_local_h::localEntity_t =
            0 as *mut crate::cg_local_h::localEntity_t;
        let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
        le = crate::src::cgame::cg_localents::CG_AllocLocalEntity();
        (*le).leFlags = crate::cg_local_h::LEF_PUFF_DONT_SCALE as i32;
        (*le).leType = crate::cg_local_h::LE_MOVE_SCALE_FADE;
        (*le).startTime = crate::src::cgame::cg_main::cg.time;
        (*le).endTime = ((crate::src::cgame::cg_main::cg.time + 1000) as f32
            + (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * 250f32)
            as i32;
        (*le).lifeRate = (1.0 / ((*le).endTime - (*le).startTime) as f64) as f32;
        re = &mut (*le).refEntity;
        (*re).shaderTime = crate::src::cgame::cg_main::cg.time as f32 / 1000.0;
        (*re).reType = crate::tr_types_h::RT_SPRITE;
        (*re).rotation = 0f32;
        (*re).radius = 3f32;
        (*re).customShader = crate::src::cgame::cg_main::cgs.media.waterBubbleShader;
        (*re).shaderRGBA[0] = 0xff;
        (*re).shaderRGBA[1] = 0xff;
        (*re).shaderRGBA[2] = 0xff;
        (*re).shaderRGBA[3] = 0xff;
        (*le).color[3] = 1f32;
        (*le).pos.trType = crate::src::qcommon::q_shared::TR_LINEAR;
        (*le).pos.trTime = crate::src::cgame::cg_main::cg.time;
        (*le).pos.trBase[0] = move_0[0];
        (*le).pos.trBase[1] = move_0[1];
        (*le).pos.trBase[2] = move_0[2];
        (*le).pos.trDelta[0] = (2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * 5f64) as crate::src::qcommon::q_shared::vec_t;
        (*le).pos.trDelta[1] = (2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * 5f64) as crate::src::qcommon::q_shared::vec_t;
        (*le).pos.trDelta[2] =
            (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 5f64
                + 6f64) as crate::src::qcommon::q_shared::vec_t;
        move_0[0] = move_0[0] + vec[0];
        move_0[1] = move_0[1] + vec[1];
        move_0[2] = move_0[2] + vec[2];
        i = (i as f32 + spacing) as i32
    }
}
/*
=====================
CG_SmokePuff

Adds a smoke puff or blood trail localEntity.
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_SmokePuff(
    mut p: *const crate::src::qcommon::q_shared::vec_t,
    mut vel: *const crate::src::qcommon::q_shared::vec_t,
    mut radius: f32,
    mut r: f32,
    mut g: f32,
    mut b: f32,
    mut a: f32,
    mut duration: f32,
    mut startTime: i32,
    mut fadeInTime: i32,
    mut leFlags: i32,
    mut hShader: crate::src::qcommon::q_shared::qhandle_t,
) -> *mut crate::cg_local_h::localEntity_t {
    static mut seed: i32 = 0x92;
    let mut le: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    //	int fadeInTime = startTime + duration / 2;
    le = crate::src::cgame::cg_localents::CG_AllocLocalEntity();
    (*le).leFlags = leFlags;
    (*le).radius = radius;
    re = &mut (*le).refEntity;
    (*re).rotation = crate::src::qcommon::q_math::Q_random(&mut seed) * 360f32;
    (*re).radius = radius;
    (*re).shaderTime = startTime as f32 / 1000.0;
    (*le).leType = crate::cg_local_h::LE_MOVE_SCALE_FADE;
    (*le).startTime = startTime;
    (*le).fadeInTime = fadeInTime;
    (*le).endTime = (startTime as f32 + duration) as i32;
    if fadeInTime > startTime {
        (*le).lifeRate = (1.0 / ((*le).endTime - (*le).fadeInTime) as f64) as f32
    } else {
        (*le).lifeRate = (1.0 / ((*le).endTime - (*le).startTime) as f64) as f32
    }
    (*le).color[0] = r;
    (*le).color[1] = g;
    (*le).color[2] = b;
    (*le).color[3] = a;
    (*le).pos.trType = crate::src::qcommon::q_shared::TR_LINEAR;
    (*le).pos.trTime = startTime;
    (*le).pos.trDelta[0] = *vel.offset(0);
    (*le).pos.trDelta[1] = *vel.offset(1);
    (*le).pos.trDelta[2] = *vel.offset(2);
    (*le).pos.trBase[0] = *p.offset(0);
    (*le).pos.trBase[1] = *p.offset(1);
    (*le).pos.trBase[2] = *p.offset(2);
    (*re).origin[0] = *p.offset(0);
    (*re).origin[1] = *p.offset(1);
    (*re).origin[2] = *p.offset(2);
    (*re).customShader = hShader;
    // rage pro can't alpha fade, so use a different shader
    if crate::src::cgame::cg_main::cgs.glconfig.hardwareType == crate::tr_types_h::GLHW_RAGEPRO {
        (*re).customShader = crate::src::cgame::cg_main::cgs.media.smokePuffRageProShader;
        (*re).shaderRGBA[0] = 0xff;
        (*re).shaderRGBA[1] = 0xff;
        (*re).shaderRGBA[2] = 0xff;
        (*re).shaderRGBA[3] = 0xff
    } else {
        (*re).shaderRGBA[0] = ((*le).color[0] * 255f32) as crate::src::qcommon::q_shared::byte;
        (*re).shaderRGBA[1] = ((*le).color[1] * 255f32) as crate::src::qcommon::q_shared::byte;
        (*re).shaderRGBA[2] = ((*le).color[2] * 255f32) as crate::src::qcommon::q_shared::byte;
        (*re).shaderRGBA[3] = 0xff
    }
    (*re).reType = crate::tr_types_h::RT_SPRITE;
    (*re).radius = (*le).radius;
    return le;
}
/*
==================
CG_SpawnEffect

Player teleporting in or out
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_SpawnEffect(mut org: *mut crate::src::qcommon::q_shared::vec_t) {
    let mut le: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    le = crate::src::cgame::cg_localents::CG_AllocLocalEntity();
    (*le).leFlags = 0;
    (*le).leType = crate::cg_local_h::LE_FADE_RGB;
    (*le).startTime = crate::src::cgame::cg_main::cg.time;
    (*le).endTime = crate::src::cgame::cg_main::cg.time + 500;
    (*le).lifeRate = (1.0 / ((*le).endTime - (*le).startTime) as f64) as f32;
    (*le).color[3] = 1f32;
    (*le).color[2] = (*le).color[3];
    (*le).color[1] = (*le).color[2];
    (*le).color[0] = (*le).color[1];
    re = &mut (*le).refEntity;
    (*re).reType = crate::tr_types_h::RT_MODEL;
    (*re).shaderTime = crate::src::cgame::cg_main::cg.time as f32 / 1000.0;
    (*re).customShader = crate::src::cgame::cg_main::cgs.media.teleportEffectShader;
    (*re).hModel = crate::src::cgame::cg_main::cgs.media.teleportEffectModel;
    crate::src::qcommon::q_math::AxisClear((*re).axis.as_mut_ptr());
    (*re).origin[0] = *org.offset(0);
    (*re).origin[1] = *org.offset(1);
    (*re).origin[2] = *org.offset(2);
    (*re).origin[2] -= 24f32;
}
/*
==================
CG_ScorePlum
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ScorePlum(
    mut client: i32,
    mut org: *mut crate::src::qcommon::q_shared::vec_t,
    mut score: i32,
) {
    let mut le: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    static mut lastPos: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // only visualize for the client that scored
    if client
        != crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .clientNum
        || crate::src::cgame::cg_main::cg_scorePlum.integer == 0
    {
        return;
    }
    le = crate::src::cgame::cg_localents::CG_AllocLocalEntity();
    (*le).leFlags = 0;
    (*le).leType = crate::cg_local_h::LE_SCOREPLUM;
    (*le).startTime = crate::src::cgame::cg_main::cg.time;
    (*le).endTime = crate::src::cgame::cg_main::cg.time + 4000;
    (*le).lifeRate = (1.0 / ((*le).endTime - (*le).startTime) as f64) as f32;
    (*le).color[3] = 1f32;
    (*le).color[2] = (*le).color[3];
    (*le).color[1] = (*le).color[2];
    (*le).color[0] = (*le).color[1];
    (*le).radius = score as f32;
    (*le).pos.trBase[0] = *org.offset(0);
    (*le).pos.trBase[1] = *org.offset(1);
    (*le).pos.trBase[2] = *org.offset(2);
    if *org.offset(2) >= lastPos[2] - 20f32 && *org.offset(2) <= lastPos[2] + 20f32 {
        (*le).pos.trBase[2] -= 20f32
    }
    //CG_Printf( "Plum origin %i %i %i -- %i\n", (int)org[0], (int)org[1], (int)org[2], (int)Distance(org, lastPos));
    lastPos[0] = *org.offset(0);
    lastPos[1] = *org.offset(1);
    lastPos[2] = *org.offset(2);
    re = &mut (*le).refEntity;
    (*re).reType = crate::tr_types_h::RT_SPRITE;
    (*re).radius = 16f32;
    angles[2] = 0f32;
    angles[1] = angles[2];
    angles[0] = angles[1];
    crate::src::qcommon::q_math::AnglesToAxis(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*re).axis.as_mut_ptr(),
    );
}
/*
====================
CG_MakeExplosion
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_MakeExplosion(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut hModel: crate::src::qcommon::q_shared::qhandle_t,
    mut shader: crate::src::qcommon::q_shared::qhandle_t,
    mut msec: i32,
    mut isSprite: crate::src::qcommon::q_shared::qboolean,
) -> *mut crate::cg_local_h::localEntity_t {
    let mut ang: f32 = 0.;
    let mut ex: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    let mut offset: i32 = 0;
    let mut tmpVec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut newOrigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if msec <= 0 {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_MakeExplosion: msec = %i\x00" as *const u8 as *const i8,
            msec,
        );
    }
    // skew the time a bit so they aren't all in sync
    offset = crate::stdlib::rand() & 63;
    ex = crate::src::cgame::cg_localents::CG_AllocLocalEntity();
    if isSprite as u64 != 0 {
        (*ex).leType = crate::cg_local_h::LE_SPRITE_EXPLOSION;
        // randomly rotate sprite orientation
        (*ex).refEntity.rotation = (crate::stdlib::rand() % 360) as f32;
        tmpVec[0] = *dir.offset(0) * 16f32;
        tmpVec[1] = *dir.offset(1) * 16f32;
        tmpVec[2] = *dir.offset(2) * 16f32;
        newOrigin[0] = tmpVec[0] + *origin.offset(0);
        newOrigin[1] = tmpVec[1] + *origin.offset(1);
        newOrigin[2] = tmpVec[2] + *origin.offset(2)
    } else {
        (*ex).leType = crate::cg_local_h::LE_EXPLOSION;
        newOrigin[0] = *origin.offset(0);
        newOrigin[1] = *origin.offset(1);
        newOrigin[2] = *origin.offset(2);
        // set axis with random rotate
        if dir.is_null() {
            crate::src::qcommon::q_math::AxisClear((*ex).refEntity.axis.as_mut_ptr());
        } else {
            ang = (crate::stdlib::rand() % 360) as f32;
            (*ex).refEntity.axis[0][0] = *dir.offset(0);
            (*ex).refEntity.axis[0][1] = *dir.offset(1);
            (*ex).refEntity.axis[0][2] = *dir.offset(2);
            crate::src::qcommon::q_math::RotateAroundDirection(
                (*ex).refEntity.axis.as_mut_ptr(),
                ang,
            );
        }
    }
    (*ex).startTime = crate::src::cgame::cg_main::cg.time - offset;
    (*ex).endTime = (*ex).startTime + msec;
    // bias the time so all shader effects start correctly
    (*ex).refEntity.shaderTime = (*ex).startTime as f32 / 1000.0;
    (*ex).refEntity.hModel = hModel;
    (*ex).refEntity.customShader = shader;
    // set origin
    (*ex).refEntity.origin[0] = newOrigin[0];
    (*ex).refEntity.origin[1] = newOrigin[1];
    (*ex).refEntity.origin[2] = newOrigin[2];
    (*ex).refEntity.oldorigin[0] = newOrigin[0];
    (*ex).refEntity.oldorigin[1] = newOrigin[1];
    (*ex).refEntity.oldorigin[2] = newOrigin[2];
    (*ex).color[2] = 1f32;
    (*ex).color[1] = (*ex).color[2];
    (*ex).color[0] = (*ex).color[1];
    return ex;
}
/*
=================
CG_Bleed

This is the spurt of blood when a character gets hit
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_Bleed(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut entityNum: i32,
) {
    let mut ex: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    if crate::src::cgame::cg_main::cg_blood.integer == 0 {
        return;
    }
    ex = crate::src::cgame::cg_localents::CG_AllocLocalEntity();
    (*ex).leType = crate::cg_local_h::LE_EXPLOSION;
    (*ex).startTime = crate::src::cgame::cg_main::cg.time;
    (*ex).endTime = (*ex).startTime + 500;
    (*ex).refEntity.origin[0] = *origin.offset(0);
    (*ex).refEntity.origin[1] = *origin.offset(1);
    (*ex).refEntity.origin[2] = *origin.offset(2);
    (*ex).refEntity.reType = crate::tr_types_h::RT_SPRITE;
    (*ex).refEntity.rotation = (crate::stdlib::rand() % 360) as f32;
    (*ex).refEntity.radius = 24f32;
    (*ex).refEntity.customShader = crate::src::cgame::cg_main::cgs.media.bloodExplosionShader;
    // don't show player's own blood in view
    if entityNum == (*crate::src::cgame::cg_main::cg.snap).ps.clientNum {
        (*ex).refEntity.renderfx |= 0x2
    };
}
/*
==================
CG_LaunchGib
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_LaunchGib(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut velocity: *mut crate::src::qcommon::q_shared::vec_t,
    mut hModel: crate::src::qcommon::q_shared::qhandle_t,
) {
    let mut le: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    le = crate::src::cgame::cg_localents::CG_AllocLocalEntity();
    re = &mut (*le).refEntity;
    (*le).leType = crate::cg_local_h::LE_FRAGMENT;
    (*le).startTime = crate::src::cgame::cg_main::cg.time;
    (*le).endTime = (((*le).startTime + 5000) as f32
        + (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * 3000f32) as i32;
    (*re).origin[0] = *origin.offset(0);
    (*re).origin[1] = *origin.offset(1);
    (*re).origin[2] = *origin.offset(2);
    crate::src::qcommon::q_math::AxisCopy(
        crate::src::qcommon::q_math::axisDefault.as_mut_ptr(),
        (*re).axis.as_mut_ptr(),
    );
    (*re).hModel = hModel;
    (*le).pos.trType = crate::src::qcommon::q_shared::TR_GRAVITY;
    (*le).pos.trBase[0] = *origin.offset(0);
    (*le).pos.trBase[1] = *origin.offset(1);
    (*le).pos.trBase[2] = *origin.offset(2);
    (*le).pos.trDelta[0] = *velocity.offset(0);
    (*le).pos.trDelta[1] = *velocity.offset(1);
    (*le).pos.trDelta[2] = *velocity.offset(2);
    (*le).pos.trTime = crate::src::cgame::cg_main::cg.time;
    (*le).bounceFactor = 0.6;
    (*le).leBounceSoundType = crate::cg_local_h::LEBS_BLOOD;
    (*le).leMarkType = crate::cg_local_h::LEMT_BLOOD;
}
#[no_mangle]

pub unsafe extern "C" fn CG_GibPlayer(mut playerOrigin: *mut crate::src::qcommon::q_shared::vec_t) {
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if crate::src::cgame::cg_main::cg_blood.integer == 0 {
        return;
    }
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[1] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (250f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 250f64)
        as crate::src::qcommon::q_shared::vec_t;
    if crate::stdlib::rand() & 1 != 0 {
        CG_LaunchGib(
            origin.as_mut_ptr(),
            velocity.as_mut_ptr(),
            crate::src::cgame::cg_main::cgs.media.gibSkull,
        );
    } else {
        CG_LaunchGib(
            origin.as_mut_ptr(),
            velocity.as_mut_ptr(),
            crate::src::cgame::cg_main::cgs.media.gibBrain,
        );
    }
    // allow gibs to be turned off for speed
    if crate::src::cgame::cg_main::cg_gibs.integer == 0 {
        return;
    }
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[1] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (250f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 250f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.gibAbdomen,
    );
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[1] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (250f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 250f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.gibArm,
    );
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[1] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (250f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 250f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.gibChest,
    );
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[1] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (250f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 250f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.gibFist,
    );
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[1] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (250f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 250f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.gibFoot,
    );
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[1] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (250f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 250f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.gibForearm,
    );
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[1] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (250f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 250f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.gibIntestine,
    );
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[1] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (250f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 250f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.gibLeg,
    );
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[1] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 250f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (250f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 250f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.gibLeg,
    );
}
/*
==================
CG_LaunchExplode
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_LaunchExplode(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut velocity: *mut crate::src::qcommon::q_shared::vec_t,
    mut hModel: crate::src::qcommon::q_shared::qhandle_t,
) {
    let mut le: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    le = crate::src::cgame::cg_localents::CG_AllocLocalEntity();
    re = &mut (*le).refEntity;
    (*le).leType = crate::cg_local_h::LE_FRAGMENT;
    (*le).startTime = crate::src::cgame::cg_main::cg.time;
    (*le).endTime = (((*le).startTime + 10000) as f32
        + (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * 6000f32) as i32;
    (*re).origin[0] = *origin.offset(0);
    (*re).origin[1] = *origin.offset(1);
    (*re).origin[2] = *origin.offset(2);
    crate::src::qcommon::q_math::AxisCopy(
        crate::src::qcommon::q_math::axisDefault.as_mut_ptr(),
        (*re).axis.as_mut_ptr(),
    );
    (*re).hModel = hModel;
    (*le).pos.trType = crate::src::qcommon::q_shared::TR_GRAVITY;
    (*le).pos.trBase[0] = *origin.offset(0);
    (*le).pos.trBase[1] = *origin.offset(1);
    (*le).pos.trBase[2] = *origin.offset(2);
    (*le).pos.trDelta[0] = *velocity.offset(0);
    (*le).pos.trDelta[1] = *velocity.offset(1);
    (*le).pos.trDelta[2] = *velocity.offset(2);
    (*le).pos.trTime = crate::src::cgame::cg_main::cg.time;
    (*le).bounceFactor = 0.1;
    (*le).leBounceSoundType = crate::cg_local_h::LEBS_BRASS;
    (*le).leMarkType = crate::cg_local_h::LEMT_NONE;
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
//
// The entire cgame module is unloaded and reloaded on each level change,
// so there is NO persistant data between levels on the client side.
// If you absolutely need something stored, it can either be kept
// by the server in the server stored userinfos, or stashed in a cvar.
// time for fragments to sink into ground before going away
// amount to scale up the icons when activating
// num frame for '-' stats digit
// very large characters
//=================================================
// player entities need to track more information
// than any other type of entity.
// note that not every player entity is a client entity,
// because corpses after respawn are outside the normal
// client numbering range
// when changing animation, set animationTime to frameTime + lerping time
// The current lerp will finish out, then it will lerp to the new animation
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// flip from 0 to 1
// machinegun spinning
//=================================================
// centity_t have a direct corespondence with gentity_t in the game, but
// only the entityState_t is directly communicated to the cgame
// from cg.frame
// from cg.nextFrame, if available
// true if next is valid to interpolate to
// true if cg.frame holds this entity
// move to playerEntity?
// so missile trails can handle dropped initial packets
// last time this entity was found in a snapshot
// decay the error from this time
// false if origin / angles is an interpolation
// exact interpolated position of entity on this frame
//======================================================================
// local entities are created as a result of events or predicted actions,
// and live independently from all server transmitted entities
// fade alpha instead of rgb
// do not scale size over time
// tumble over time, used for ejecting shells
// sound 1 for kamikaze
// sound 2 for kamikaze
// fragment local entities can leave marks on walls
// fragment local entities can make sounds on impacts
// 1.0 / (endTime - startTime)
// 0.0 = no bounce, 1.0 = perfect
// mark to leave on fragment impact
//======================================================================
// each client has an associated clientInfo_t
// that contains media references necessary to present the
// client model and other color coded effects
// this is regenerated each time a client's configstring changes,
// usually as a result of a userinfo (name, model, etc) change
// 0 = not bot, 1-5 = bot
// updated by score servercmds
// location index for team mode
// you only get this info about your teammates
// in tourney mode
// task in teamplay (offence/defence)
// true when this is a team leader
// so can display quad/flag status
// when clientinfo is changed, the loading of models/skins/sounds
// can be deferred until you are dead, to prevent hitches in
// gameplay
// true if using the new mission pack animations
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// move head in icon views
// from model
// each WP_* weapon enum has an associated weaponInfo_t
// that contains media references necessary to present the
// weapon and its effects
// the hands don't actually draw, they just position the weapon
// so it will rotate centered instead of by tag
// fast firing weapons randomly choose
// each IT_* item has an associated itemInfo_t
// that constains media references necessary to present the
// item and its effects
//======================================================================
// all cg.stepTime, cg.duckTime, cg.landTime, etc are set to cg.time when the action
// occurs, and they will have visible effects for #define STEP_TIME or whatever msec after
// incremented each frame
// taking a level menu screenshot
// don't defer players at initial startup
// don't play voice rewards, because game will end shortly
// there are only one or two snapshot_t that are relevant at a time
// the number of snapshots the client system has received
// the time from latestSnapshotNum, so we don't need to read the snapshot yet
// cg.snap->serverTime <= cg.time
// cg.nextSnap->serverTime > cg.time, or NULL
// (float)( cg.time - cg.frame->serverTime ) / (cg.nextFrame->serverTime - cg.frame->serverTime)
// cg.time - cg.oldTime
// this is the time value that the client
// is rendering at.
// time at last frame, used for missile trails and prediction checking
// either cg.snap->time or cg.nextSnap->time
// 5 min, 1 min, overtime
// set on a map restart to set back the weapon
// during deaths, chasecams, etc
// prediction state
// true if prediction has hit a trigger_teleport
// clear until the first call to CG_PredictPlayerState
// for stair up smoothing
// for duck viewheight smoothing
// for landing hard
// input state sent to server
// auto rotating items
// view rendering
// will be converted to refdef.viewaxis
// zoom key
// information screen text during loading
// scoreboard
// list of names
// length of list
// width in device units
// next time to offset
// current paint x
// current paint x
// current offset from start
// current offset from start
// centerprinting
// low ammo warning state
// 1 = low, 2 = empty
// crosshair client ID
// powerup active flashing
// attacking player
// reward medals
// sound buffer mainly for announcer sounds
// warmup countdown
//==========================
// the pulse around the crosshair is timed separately
// blend blobs
// status bar head
// view movement
// temp working variables for player view
//qboolean cameraMode;		// if rendering from a loaded camera
// development tool
// all of the model, shader, and sound references that are
// loaded at gamestate time are stored in cgMedia_t
// Other media that can be tied to clients, weapons, or items are
// stored in the clientInfo_t, itemInfo_t, weaponInfo_t, and powerupInfo_t
// gib explosions
// wall mark shaders
// powerup shaders
// weapon effect models
// weapon effect shaders
// special effects models
// scoreboard headers
// medals shown during gameplay
// sounds
//sfxHandle_t	sfx_railg;
// teamplay sounds
// tournament sounds
// The client game static (cgs) structure hold everything
// loaded or calculated from the gamestate.  It will NOT
// be cleared when a tournement restart is done, allowing
// all clients to begin playing instantly
// gamestate from server
// rendering configuration
// derived from glconfig
// reliable command stream counter
// the number of snapshots cgame has requested
// detected on startup by checking sv_running
// parsed from serverinfo
// beep whenever changed
// beep whenever changed
// from configstrings
// flag status from configstrings
//
// locally derived information from gamestate
//
// teamchat width is *3 because of embedded color codes
// orders
// media
//==============================================================================
//extern	vmCvar_t		cg_pmove_fixed;
//
// cg_main.c
//
//
// cg_view.c
//
//
// cg_drawtools.c
//
//
// cg_draw.c, cg_newDraw.c
//
//
// cg_player.c
//
//
// cg_predict.c
//
//
// cg_events.c
//
//
// cg_ents.c
//
//
// cg_weapons.c
//
// should this be in pmove?
//
// cg_marks.c
//
//
// cg_localents.c
//
//
// cg_effects.c
//
/*
===================
CG_BigExplode

Generated a bunch of gibs launching out from the bodies location
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_BigExplode(
    mut playerOrigin: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if crate::src::cgame::cg_main::cg_blood.integer == 0 {
        return;
    }
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 100f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[1] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 100f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (150f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 100f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchExplode(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.smoke2,
    );
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 100f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[1] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 100f64) as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (150f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 100f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchExplode(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.smoke2,
    );
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] =
        (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 100f64 * 1.5)
            as crate::src::qcommon::q_shared::vec_t;
    velocity[1] =
        (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 100f64 * 1.5)
            as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (150f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 100f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchExplode(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.smoke2,
    );
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] =
        (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 100f64 * 2.0)
            as crate::src::qcommon::q_shared::vec_t;
    velocity[1] =
        (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 100f64 * 2.0)
            as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (150f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 100f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchExplode(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.smoke2,
    );
    origin[0] = *playerOrigin.offset(0);
    origin[1] = *playerOrigin.offset(1);
    origin[2] = *playerOrigin.offset(2);
    velocity[0] =
        (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 100f64 * 2.5)
            as crate::src::qcommon::q_shared::vec_t;
    velocity[1] =
        (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 100f64 * 2.5)
            as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (150f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 100f64)
        as crate::src::qcommon::q_shared::vec_t;
    CG_LaunchExplode(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.media.smoke2,
    );
}
