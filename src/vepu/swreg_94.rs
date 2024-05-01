#[doc = "Register `SWREG_94` reader"]
pub type R = crate::R<Swreg94Spec>;
#[doc = "Register `SWREG_94` writer"]
pub type W = crate::W<Swreg94Spec>;
#[doc = "Field `STAB_HOR_GMV` reader - the horizontal output of Stabilization GMV\n\nsigned register\n\nrange : -16~16"]
pub type StabHorGmvR = crate::FieldReader;
#[doc = "Field `STAB_HOR_GMV` writer - the horizontal output of Stabilization GMV\n\nsigned register\n\nrange : -16~16"]
pub type StabHorGmvW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "the mode select of Stabilization\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StabModSel {
    #[doc = "0: disabled"]
    D0 = 0,
    #[doc = "1: stab only"]
    D1 = 1,
    #[doc = "2: stab+encode"]
    D2 = 2,
}
impl From<StabModSel> for u8 {
    #[inline(always)]
    fn from(variant: StabModSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for StabModSel {
    type Ux = u8;
}
#[doc = "Field `STAB_MOD_SEL` reader - the mode select of Stabilization"]
pub type StabModSelR = crate::FieldReader<StabModSel>;
impl StabModSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<StabModSel> {
        match self.bits {
            0 => Some(StabModSel::D0),
            1 => Some(StabModSel::D1),
            2 => Some(StabModSel::D2),
            _ => None,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == StabModSel::D0
    }
    #[doc = "stab only"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == StabModSel::D1
    }
    #[doc = "stab+encode"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == StabModSel::D2
    }
}
#[doc = "Field `STAB_MOD_SEL` writer - the mode select of Stabilization"]
pub type StabModSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, StabModSel>;
impl<'a, REG> StabModSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(StabModSel::D0)
    }
    #[doc = "stab only"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(StabModSel::D1)
    }
    #[doc = "stab+encode"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(StabModSel::D2)
    }
}
#[doc = "Field `STAB_MIN_VALUE` reader - the minimum value output of Stabilization\n\nrange : 0~255*253*253"]
pub type StabMinValueR = crate::FieldReader<u32>;
#[doc = "Field `STAB_MIN_VALUE` writer - the minimum value output of Stabilization\n\nrange : 0~255*253*253"]
pub type StabMinValueW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:5 - the horizontal output of Stabilization GMV\n\nsigned register\n\nrange : -16~16"]
    #[inline(always)]
    pub fn stab_hor_gmv(&self) -> StabHorGmvR {
        StabHorGmvR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - the mode select of Stabilization"]
    #[inline(always)]
    pub fn stab_mod_sel(&self) -> StabModSelR {
        StabModSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - the minimum value output of Stabilization\n\nrange : 0~255*253*253"]
    #[inline(always)]
    pub fn stab_min_value(&self) -> StabMinValueR {
        StabMinValueR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - the horizontal output of Stabilization GMV\n\nsigned register\n\nrange : -16~16"]
    #[inline(always)]
    #[must_use]
    pub fn stab_hor_gmv(&mut self) -> StabHorGmvW<Swreg94Spec> {
        StabHorGmvW::new(self, 0)
    }
    #[doc = "Bits 6:7 - the mode select of Stabilization"]
    #[inline(always)]
    #[must_use]
    pub fn stab_mod_sel(&mut self) -> StabModSelW<Swreg94Spec> {
        StabModSelW::new(self, 6)
    }
    #[doc = "Bits 8:31 - the minimum value output of Stabilization\n\nrange : 0~255*253*253"]
    #[inline(always)]
    #[must_use]
    pub fn stab_min_value(&mut self) -> StabMinValueW<Swreg94Spec> {
        StabMinValueW::new(self, 8)
    }
}
#[doc = "output of Stabilization\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_94::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_94::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg94Spec;
impl crate::RegisterSpec for Swreg94Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_94::R`](R) reader structure"]
impl crate::Readable for Swreg94Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_94::W`](W) writer structure"]
impl crate::Writable for Swreg94Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_94 to value 0"]
impl crate::Resettable for Swreg94Spec {
    const RESET_VALUE: u32 = 0;
}
