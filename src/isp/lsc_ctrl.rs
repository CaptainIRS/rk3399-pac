#[doc = "Register `LSC_CTRL` reader"]
pub type R = crate::R<LscCtrlSpec>;
#[doc = "Register `LSC_CTRL` writer"]
pub type W = crate::W<LscCtrlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LscEn {
    #[doc = "0: activation request for lens shading correction"]
    B0 = 0,
    #[doc = "1: deactivation reqeust for lens shading correction Activation/Deactivation is object of a shadowing mechnism. The current status is visible at ISP_LSC_STATUS::lsc_enable_status"]
    B1 = 1,
}
impl From<LscEn> for bool {
    #[inline(always)]
    fn from(variant: LscEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `lsc_en` reader - "]
pub type LscEnR = crate::BitReader<LscEn>;
impl LscEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LscEn {
        match self.bits {
            false => LscEn::B0,
            true => LscEn::B1,
        }
    }
    #[doc = "activation request for lens shading correction"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LscEn::B0
    }
    #[doc = "deactivation reqeust for lens shading correction Activation/Deactivation is object of a shadowing mechnism. The current status is visible at ISP_LSC_STATUS::lsc_enable_status"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LscEn::B1
    }
}
#[doc = "Field `lsc_en` writer - "]
pub type LscEnW<'a, REG> = crate::BitWriter<'a, REG, LscEn>;
impl<'a, REG> LscEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "activation request for lens shading correction"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LscEn::B0)
    }
    #[doc = "deactivation reqeust for lens shading correction Activation/Deactivation is object of a shadowing mechnism. The current status is visible at ISP_LSC_STATUS::lsc_enable_status"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LscEn::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lsc_en(&self) -> LscEnR {
        LscEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn lsc_en(&mut self) -> LscEnW<LscCtrlSpec> {
        LscEnW::new(self, 0)
    }
}
#[doc = "Lens shade control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscCtrlSpec;
impl crate::RegisterSpec for LscCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_ctrl::R`](R) reader structure"]
impl crate::Readable for LscCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lsc_ctrl::W`](W) writer structure"]
impl crate::Writable for LscCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_CTRL to value 0"]
impl crate::Resettable for LscCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
