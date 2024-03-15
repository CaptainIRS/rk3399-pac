#[doc = "Register `AUX_CH_CTL_2` reader"]
pub type R = crate::R<AuxChCtl2Spec>;
#[doc = "Register `AUX_CH_CTL_2` writer"]
pub type W = crate::W<AuxChCtl2Spec>;
#[doc = "Field `AUX_EN` reader - Register control AUX CH operation enable Write 1 to this bit to enable AUX CH operation This bit will self-clear when AUX CH operation is finished. This bit is self cleared."]
pub type AuxEnR = crate::BitReader;
#[doc = "Field `AUX_EN` writer - Register control AUX CH operation enable Write 1 to this bit to enable AUX CH operation This bit will self-clear when AUX CH operation is finished. This bit is self cleared."]
pub type AuxEnW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "AUX CH issue “address only” command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrOnly {
    #[doc = "1: Normal AUX CH command"]
    B1 = 1,
    #[doc = "0: Normal AUX CH command"]
    B0 = 0,
}
impl From<AddrOnly> for bool {
    #[inline(always)]
    fn from(variant: AddrOnly) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR_ONLY` reader - AUX CH issue “address only” command"]
pub type AddrOnlyR = crate::BitReader<AddrOnly>;
impl AddrOnlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddrOnly {
        match self.bits {
            true => AddrOnly::B1,
            false => AddrOnly::B0,
        }
    }
    #[doc = "Normal AUX CH command"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AddrOnly::B1
    }
    #[doc = "Normal AUX CH command"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AddrOnly::B0
    }
}
#[doc = "Field `ADDR_ONLY` writer - AUX CH issue “address only” command"]
pub type AddrOnlyW<'a, REG> = crate::BitWriter<'a, REG, AddrOnly>;
impl<'a, REG> AddrOnlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal AUX CH command"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AddrOnly::B1)
    }
    #[doc = "Normal AUX CH command"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AddrOnly::B0)
    }
}
#[doc = "Invert AUX CH PN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxPnInv {
    #[doc = "1: Normal mode"]
    B1 = 1,
    #[doc = "0: Normal mode"]
    B0 = 0,
}
impl From<AuxPnInv> for bool {
    #[inline(always)]
    fn from(variant: AuxPnInv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_PN_INV` reader - Invert AUX CH PN"]
pub type AuxPnInvR = crate::BitReader<AuxPnInv>;
impl AuxPnInvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxPnInv {
        match self.bits {
            true => AuxPnInv::B1,
            false => AuxPnInv::B0,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AuxPnInv::B1
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AuxPnInv::B0
    }
}
#[doc = "Field `AUX_PN_INV` writer - Invert AUX CH PN"]
pub type AuxPnInvW<'a, REG> = crate::BitWriter<'a, REG, AuxPnInv>;
impl<'a, REG> AuxPnInvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AuxPnInv::B1)
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AuxPnInv::B0)
    }
}
#[doc = "Power down AUX CH when AUX CH is in idle state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdAuxIdle {
    #[doc = "1: Keep AUX CH power up in idle state."]
    B1 = 1,
    #[doc = "0: Keep AUX CH power up in idle state."]
    B0 = 0,
}
impl From<PdAuxIdle> for bool {
    #[inline(always)]
    fn from(variant: PdAuxIdle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_AUX_IDLE` reader - Power down AUX CH when AUX CH is in idle state."]
pub type PdAuxIdleR = crate::BitReader<PdAuxIdle>;
impl PdAuxIdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdAuxIdle {
        match self.bits {
            true => PdAuxIdle::B1,
            false => PdAuxIdle::B0,
        }
    }
    #[doc = "Keep AUX CH power up in idle state."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PdAuxIdle::B1
    }
    #[doc = "Keep AUX CH power up in idle state."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PdAuxIdle::B0
    }
}
#[doc = "Field `PD_AUX_IDLE` writer - Power down AUX CH when AUX CH is in idle state."]
pub type PdAuxIdleW<'a, REG> = crate::BitWriter<'a, REG, PdAuxIdle>;
impl<'a, REG> PdAuxIdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Keep AUX CH power up in idle state."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PdAuxIdle::B1)
    }
    #[doc = "Keep AUX CH power up in idle state."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PdAuxIdle::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Register control AUX CH operation enable Write 1 to this bit to enable AUX CH operation This bit will self-clear when AUX CH operation is finished. This bit is self cleared."]
    #[inline(always)]
    pub fn aux_en(&self) -> AuxEnR {
        AuxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AUX CH issue “address only” command"]
    #[inline(always)]
    pub fn addr_only(&self) -> AddrOnlyR {
        AddrOnlyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Invert AUX CH PN"]
    #[inline(always)]
    pub fn aux_pn_inv(&self) -> AuxPnInvR {
        AuxPnInvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power down AUX CH when AUX CH is in idle state."]
    #[inline(always)]
    pub fn pd_aux_idle(&self) -> PdAuxIdleR {
        PdAuxIdleR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register control AUX CH operation enable Write 1 to this bit to enable AUX CH operation This bit will self-clear when AUX CH operation is finished. This bit is self cleared."]
    #[inline(always)]
    #[must_use]
    pub fn aux_en(&mut self) -> AuxEnW<AuxChCtl2Spec> {
        AuxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - AUX CH issue “address only” command"]
    #[inline(always)]
    #[must_use]
    pub fn addr_only(&mut self) -> AddrOnlyW<AuxChCtl2Spec> {
        AddrOnlyW::new(self, 1)
    }
    #[doc = "Bit 2 - Invert AUX CH PN"]
    #[inline(always)]
    #[must_use]
    pub fn aux_pn_inv(&mut self) -> AuxPnInvW<AuxChCtl2Spec> {
        AuxPnInvW::new(self, 2)
    }
    #[doc = "Bit 3 - Power down AUX CH when AUX CH is in idle state."]
    #[inline(always)]
    #[must_use]
    pub fn pd_aux_idle(&mut self) -> PdAuxIdleW<AuxChCtl2Spec> {
        PdAuxIdleW::new(self, 3)
    }
}
#[doc = "DP AUX CH Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_ch_ctl_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_ch_ctl_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxChCtl2Spec;
impl crate::RegisterSpec for AuxChCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_ch_ctl_2::R`](R) reader structure"]
impl crate::Readable for AuxChCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`aux_ch_ctl_2::W`](W) writer structure"]
impl crate::Writable for AuxChCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets AUX_CH_CTL_2 to value 0"]
impl crate::Resettable for AuxChCtl2Spec {
    const RESET_VALUE: u32 = 0;
}
