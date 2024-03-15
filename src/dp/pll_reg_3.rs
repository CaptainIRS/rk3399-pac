#[doc = "Register `PLL_REG_3` reader"]
pub type R = crate::R<PllReg3Spec>;
#[doc = "Register `PLL_REG_3` writer"]
pub type W = crate::W<PllReg3Spec>;
#[doc = "force PLL lock detector lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllLockDetForce {
    #[doc = "0: force PLL lock"]
    B0 = 0,
    #[doc = "1: force PLL lock"]
    B1 = 1,
}
impl From<PllLockDetForce> for bool {
    #[inline(always)]
    fn from(variant: PllLockDetForce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_LOCK_DET_FORCE` reader - force PLL lock detector lock"]
pub type PllLockDetForceR = crate::BitReader<PllLockDetForce>;
impl PllLockDetForceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllLockDetForce {
        match self.bits {
            false => PllLockDetForce::B0,
            true => PllLockDetForce::B1,
        }
    }
    #[doc = "force PLL lock"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PllLockDetForce::B0
    }
    #[doc = "force PLL lock"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PllLockDetForce::B1
    }
}
#[doc = "Field `PLL_LOCK_DET_FORCE` writer - force PLL lock detector lock"]
pub type PllLockDetForceW<'a, REG> = crate::BitWriter1C<'a, REG, PllLockDetForce>;
impl<'a, REG> PllLockDetForceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "force PLL lock"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PllLockDetForce::B0)
    }
    #[doc = "force PLL lock"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PllLockDetForce::B1)
    }
}
#[doc = "PLL lock detector mode select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllLockDetMode {
    #[doc = "0: integer N"]
    B0 = 0,
    #[doc = "1: integer N"]
    B1 = 1,
}
impl From<PllLockDetMode> for bool {
    #[inline(always)]
    fn from(variant: PllLockDetMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_LOCK_DET_MODE` reader - PLL lock detector mode select"]
pub type PllLockDetModeR = crate::BitReader<PllLockDetMode>;
impl PllLockDetModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllLockDetMode {
        match self.bits {
            false => PllLockDetMode::B0,
            true => PllLockDetMode::B1,
        }
    }
    #[doc = "integer N"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PllLockDetMode::B0
    }
    #[doc = "integer N"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PllLockDetMode::B1
    }
}
#[doc = "Field `PLL_LOCK_DET_MODE` writer - PLL lock detector mode select"]
pub type PllLockDetModeW<'a, REG> = crate::BitWriter1C<'a, REG, PllLockDetMode>;
impl<'a, REG> PllLockDetModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "integer N"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PllLockDetMode::B0)
    }
    #[doc = "integer N"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PllLockDetMode::B1)
    }
}
#[doc = "lock detector bypass select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LockDetBypass {
    #[doc = "0: bypass lock detector in ssc"]
    B0 = 0,
    #[doc = "1: bypass lock detector in ssc"]
    B1 = 1,
}
impl From<LockDetBypass> for bool {
    #[inline(always)]
    fn from(variant: LockDetBypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK_DET_BYPASS` reader - lock detector bypass select"]
pub type LockDetBypassR = crate::BitReader<LockDetBypass>;
impl LockDetBypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LockDetBypass {
        match self.bits {
            false => LockDetBypass::B0,
            true => LockDetBypass::B1,
        }
    }
    #[doc = "bypass lock detector in ssc"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LockDetBypass::B0
    }
    #[doc = "bypass lock detector in ssc"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LockDetBypass::B1
    }
}
#[doc = "Field `LOCK_DET_BYPASS` writer - lock detector bypass select"]
pub type LockDetBypassW<'a, REG> = crate::BitWriter1C<'a, REG, LockDetBypass>;
impl<'a, REG> LockDetBypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bypass lock detector in ssc"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LockDetBypass::B0)
    }
    #[doc = "bypass lock detector in ssc"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LockDetBypass::B1)
    }
}
#[doc = "PLL and ssc reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PallSscReset {
    #[doc = "1: normal"]
    B1 = 1,
    #[doc = "0: normal"]
    B0 = 0,
}
impl From<PallSscReset> for bool {
    #[inline(always)]
    fn from(variant: PallSscReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PALL_SSC_RESET` reader - PLL and ssc reset control"]
pub type PallSscResetR = crate::BitReader<PallSscReset>;
impl PallSscResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PallSscReset {
        match self.bits {
            true => PallSscReset::B1,
            false => PallSscReset::B0,
        }
    }
    #[doc = "normal"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PallSscReset::B1
    }
    #[doc = "normal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PallSscReset::B0
    }
}
#[doc = "Field `PALL_SSC_RESET` writer - PLL and ssc reset control"]
pub type PallSscResetW<'a, REG> = crate::BitWriter1C<'a, REG, PallSscReset>;
impl<'a, REG> PallSscResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PallSscReset::B1)
    }
    #[doc = "normal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PallSscReset::B0)
    }
}
#[doc = "loop filter control voltage reset select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopFilterResetSel {
    #[doc = "1: reset to DVDD (default)"]
    B1 = 1,
    #[doc = "0: reset to DVDD (default)"]
    B0 = 0,
}
impl From<LoopFilterResetSel> for bool {
    #[inline(always)]
    fn from(variant: LoopFilterResetSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOP_FILTER_RESET_SEL` reader - loop filter control voltage reset select"]
pub type LoopFilterResetSelR = crate::BitReader<LoopFilterResetSel>;
impl LoopFilterResetSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LoopFilterResetSel {
        match self.bits {
            true => LoopFilterResetSel::B1,
            false => LoopFilterResetSel::B0,
        }
    }
    #[doc = "reset to DVDD (default)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LoopFilterResetSel::B1
    }
    #[doc = "reset to DVDD (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LoopFilterResetSel::B0
    }
}
#[doc = "Field `LOOP_FILTER_RESET_SEL` writer - loop filter control voltage reset select"]
pub type LoopFilterResetSelW<'a, REG> = crate::BitWriter<'a, REG, LoopFilterResetSel>;
impl<'a, REG> LoopFilterResetSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "reset to DVDD (default)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LoopFilterResetSel::B1)
    }
    #[doc = "reset to DVDD (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LoopFilterResetSel::B0)
    }
}
#[doc = "lock detector output counter select, counter period is twice of reference clock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockDetCntSel {
    #[doc = "0: 512 cycle"]
    B00 = 0,
    #[doc = "1: 512 cycle"]
    B01 = 1,
    #[doc = "2: 512 cycle"]
    B10 = 2,
    #[doc = "3: 512 cycle"]
    B11 = 3,
}
impl From<LockDetCntSel> for u8 {
    #[inline(always)]
    fn from(variant: LockDetCntSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockDetCntSel {
    type Ux = u8;
}
#[doc = "Field `LOCK_DET_CNT_SEL` reader - lock detector output counter select, counter period is twice of reference clock"]
pub type LockDetCntSelR = crate::FieldReader<LockDetCntSel>;
impl LockDetCntSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LockDetCntSel {
        match self.bits {
            0 => LockDetCntSel::B00,
            1 => LockDetCntSel::B01,
            2 => LockDetCntSel::B10,
            3 => LockDetCntSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "512 cycle"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == LockDetCntSel::B00
    }
    #[doc = "512 cycle"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == LockDetCntSel::B01
    }
    #[doc = "512 cycle"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == LockDetCntSel::B10
    }
    #[doc = "512 cycle"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == LockDetCntSel::B11
    }
}
#[doc = "Field `LOCK_DET_CNT_SEL` writer - lock detector output counter select, counter period is twice of reference clock"]
pub type LockDetCntSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LockDetCntSel>;
impl<'a, REG> LockDetCntSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "512 cycle"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(LockDetCntSel::B00)
    }
    #[doc = "512 cycle"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(LockDetCntSel::B01)
    }
    #[doc = "512 cycle"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(LockDetCntSel::B10)
    }
    #[doc = "512 cycle"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(LockDetCntSel::B11)
    }
}
impl R {
    #[doc = "Bit 0 - force PLL lock detector lock"]
    #[inline(always)]
    pub fn pll_lock_det_force(&self) -> PllLockDetForceR {
        PllLockDetForceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL lock detector mode select"]
    #[inline(always)]
    pub fn pll_lock_det_mode(&self) -> PllLockDetModeR {
        PllLockDetModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - lock detector bypass select"]
    #[inline(always)]
    pub fn lock_det_bypass(&self) -> LockDetBypassR {
        LockDetBypassR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PLL and ssc reset control"]
    #[inline(always)]
    pub fn pall_ssc_reset(&self) -> PallSscResetR {
        PallSscResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - loop filter control voltage reset select"]
    #[inline(always)]
    pub fn loop_filter_reset_sel(&self) -> LoopFilterResetSelR {
        LoopFilterResetSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - lock detector output counter select, counter period is twice of reference clock"]
    #[inline(always)]
    pub fn lock_det_cnt_sel(&self) -> LockDetCntSelR {
        LockDetCntSelR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - force PLL lock detector lock"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock_det_force(&mut self) -> PllLockDetForceW<PllReg3Spec> {
        PllLockDetForceW::new(self, 0)
    }
    #[doc = "Bit 1 - PLL lock detector mode select"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock_det_mode(&mut self) -> PllLockDetModeW<PllReg3Spec> {
        PllLockDetModeW::new(self, 1)
    }
    #[doc = "Bit 2 - lock detector bypass select"]
    #[inline(always)]
    #[must_use]
    pub fn lock_det_bypass(&mut self) -> LockDetBypassW<PllReg3Spec> {
        LockDetBypassW::new(self, 2)
    }
    #[doc = "Bit 3 - PLL and ssc reset control"]
    #[inline(always)]
    #[must_use]
    pub fn pall_ssc_reset(&mut self) -> PallSscResetW<PllReg3Spec> {
        PallSscResetW::new(self, 3)
    }
    #[doc = "Bit 4 - loop filter control voltage reset select"]
    #[inline(always)]
    #[must_use]
    pub fn loop_filter_reset_sel(&mut self) -> LoopFilterResetSelW<PllReg3Spec> {
        LoopFilterResetSelW::new(self, 4)
    }
    #[doc = "Bits 5:6 - lock detector output counter select, counter period is twice of reference clock"]
    #[inline(always)]
    #[must_use]
    pub fn lock_det_cnt_sel(&mut self) -> LockDetCntSelW<PllReg3Spec> {
        LockDetCntSelW::new(self, 5)
    }
}
#[doc = "Pll_control_3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_reg_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_reg_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllReg3Spec;
impl crate::RegisterSpec for PllReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_reg_3::R`](R) reader structure"]
impl crate::Readable for PllReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pll_reg_3::W`](W) writer structure"]
impl crate::Writable for PllReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x6f;
}
#[doc = "`reset()` method sets PLL_REG_3 to value 0x2b"]
impl crate::Resettable for PllReg3Spec {
    const RESET_VALUE: u32 = 0x2b;
}
