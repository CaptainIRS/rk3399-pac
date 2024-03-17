#[doc = "Register `DENALI_CTL_32` reader"]
pub type R = crate::R<DenaliCtl32Spec>;
#[doc = "Register `DENALI_CTL_32` writer"]
pub type W = crate::W<DenaliCtl32Spec>;
#[doc = "Field `TMRD_F0` reader - DRAM TMRD value for frequency copy 0 in cycles."]
pub type TmrdF0R = crate::FieldReader;
#[doc = "Field `TMRD_F0` writer - DRAM TMRD value for frequency copy 0 in cycles."]
pub type TmrdF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMOD_F0` reader - DRAM TMOD value for frequency copy 0 in cycles."]
pub type TmodF0R = crate::FieldReader;
#[doc = "Field `TMOD_F0` writer - DRAM TMOD value for frequency copy 0 in cycles."]
pub type TmodF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DRAM TMRD value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tmrd_f0(&self) -> TmrdF0R {
        TmrdF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DRAM TMOD value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tmod_f0(&self) -> TmodF0R {
        TmodF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DRAM TMRD value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrd_f0(&mut self) -> TmrdF0W<DenaliCtl32Spec> {
        TmrdF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TMOD value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmod_f0(&mut self) -> TmodF0W<DenaliCtl32Spec> {
        TmodF0W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl32Spec;
impl crate::RegisterSpec for DenaliCtl32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_32::R`](R) reader structure"]
impl crate::Readable for DenaliCtl32Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_32::W`](W) writer structure"]
impl crate::Writable for DenaliCtl32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_32 to value 0"]
impl crate::Resettable for DenaliCtl32Spec {
    const RESET_VALUE: u32 = 0;
}
