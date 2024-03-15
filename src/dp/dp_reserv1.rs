#[doc = "Register `DP_RESERV1` reader"]
pub type R = crate::R<DpReserv1Spec>;
#[doc = "Register `DP_RESERV1` writer"]
pub type W = crate::W<DpReserv1Spec>;
#[doc = "Field `ATE_EN_CH0` reader - Set 1 to enable CH0 ATE test"]
pub type AteEnCh0R = crate::BitReader;
#[doc = "Field `ATE_EN_CH0` writer - Set 1 to enable CH0 ATE test"]
pub type AteEnCh0W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ATE_EN_CH1` reader - Set 1 to enable CH1 ATE test"]
pub type AteEnCh1R = crate::BitReader;
#[doc = "Field `ATE_EN_CH1` writer - Set 1 to enable CH1 ATE test"]
pub type AteEnCh1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ATE_EN_CH2` reader - Set 1 to enable CH2 ATE test"]
pub type AteEnCh2R = crate::BitReader;
#[doc = "Field `ATE_EN_CH2` writer - Set 1 to enable CH2 ATE test"]
pub type AteEnCh2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ATE_EN_CH3` reader - Set 1 to enable CH3 ATE test"]
pub type AteEnCh3R = crate::BitReader;
#[doc = "Field `ATE_EN_CH3` writer - Set 1 to enable CH3 ATE test"]
pub type AteEnCh3W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Pre-driver extra power control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PreDriverPwCtrl2 {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PreDriverPwCtrl2> for u8 {
    #[inline(always)]
    fn from(variant: PreDriverPwCtrl2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PreDriverPwCtrl2 {
    type Ux = u8;
}
#[doc = "Field `PRE_DRIVER_PW_CTRL2` reader - Pre-driver extra power control"]
pub type PreDriverPwCtrl2R = crate::FieldReader<PreDriverPwCtrl2>;
impl PreDriverPwCtrl2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PreDriverPwCtrl2> {
        match self.bits {
            0 => Some(PreDriverPwCtrl2::B0),
            1 => Some(PreDriverPwCtrl2::B1),
            _ => None,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PreDriverPwCtrl2::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PreDriverPwCtrl2::B1
    }
}
#[doc = "Field `PRE_DRIVER_PW_CTRL2` writer - Pre-driver extra power control"]
pub type PreDriverPwCtrl2W<'a, REG> = crate::FieldWriter<'a, REG, 2, PreDriverPwCtrl2>;
impl<'a, REG> PreDriverPwCtrl2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PreDriverPwCtrl2::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PreDriverPwCtrl2::B1)
    }
}
#[doc = "Field `SSC_MODE_LOCK` reader - SSC mode lock"]
pub type SscModeLockR = crate::BitReader;
#[doc = "Field `SSC_MODE_LOCK` writer - SSC mode lock"]
pub type SscModeLockW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable CH0 ATE test"]
    #[inline(always)]
    pub fn ate_en_ch0(&self) -> AteEnCh0R {
        AteEnCh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to enable CH1 ATE test"]
    #[inline(always)]
    pub fn ate_en_ch1(&self) -> AteEnCh1R {
        AteEnCh1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable CH2 ATE test"]
    #[inline(always)]
    pub fn ate_en_ch2(&self) -> AteEnCh2R {
        AteEnCh2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to enable CH3 ATE test"]
    #[inline(always)]
    pub fn ate_en_ch3(&self) -> AteEnCh3R {
        AteEnCh3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pre-driver extra power control"]
    #[inline(always)]
    pub fn pre_driver_pw_ctrl2(&self) -> PreDriverPwCtrl2R {
        PreDriverPwCtrl2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - SSC mode lock"]
    #[inline(always)]
    pub fn ssc_mode_lock(&self) -> SscModeLockR {
        SscModeLockR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable CH0 ATE test"]
    #[inline(always)]
    #[must_use]
    pub fn ate_en_ch0(&mut self) -> AteEnCh0W<DpReserv1Spec> {
        AteEnCh0W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to enable CH1 ATE test"]
    #[inline(always)]
    #[must_use]
    pub fn ate_en_ch1(&mut self) -> AteEnCh1W<DpReserv1Spec> {
        AteEnCh1W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 1 to enable CH2 ATE test"]
    #[inline(always)]
    #[must_use]
    pub fn ate_en_ch2(&mut self) -> AteEnCh2W<DpReserv1Spec> {
        AteEnCh2W::new(self, 2)
    }
    #[doc = "Bit 3 - Set 1 to enable CH3 ATE test"]
    #[inline(always)]
    #[must_use]
    pub fn ate_en_ch3(&mut self) -> AteEnCh3W<DpReserv1Spec> {
        AteEnCh3W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Pre-driver extra power control"]
    #[inline(always)]
    #[must_use]
    pub fn pre_driver_pw_ctrl2(&mut self) -> PreDriverPwCtrl2W<DpReserv1Spec> {
        PreDriverPwCtrl2W::new(self, 4)
    }
    #[doc = "Bit 7 - SSC mode lock"]
    #[inline(always)]
    #[must_use]
    pub fn ssc_mode_lock(&mut self) -> SscModeLockW<DpReserv1Spec> {
        SscModeLockW::new(self, 7)
    }
}
#[doc = "RESERVD1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_reserv1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_reserv1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpReserv1Spec;
impl crate::RegisterSpec for DpReserv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_reserv1::R`](R) reader structure"]
impl crate::Readable for DpReserv1Spec {}
#[doc = "`write(|w| ..)` method takes [`dp_reserv1::W`](W) writer structure"]
impl crate::Writable for DpReserv1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xbf;
}
#[doc = "`reset()` method sets DP_RESERV1 to value 0"]
impl crate::Resettable for DpReserv1Spec {
    const RESET_VALUE: u32 = 0;
}
