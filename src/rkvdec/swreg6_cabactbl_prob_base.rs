#[doc = "Register `SWREG6_CABACTBL_PROB_BASE` reader"]
pub type R = crate::R<Swreg6CabactblProbBaseSpec>;
#[doc = "Register `SWREG6_CABACTBL_PROB_BASE` writer"]
pub type W = crate::W<Swreg6CabactblProbBaseSpec>;
#[doc = "Field `SW_CABACTBL_BASE` reader - the base address of cabac table\n\nthe base address of cabac table\n\nthe address should 128bit align"]
pub type SwCabactblBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_CABACTBL_BASE` writer - the base address of cabac table\n\nthe base address of cabac table\n\nthe address should 128bit align"]
pub type SwCabactblBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - the base address of cabac table\n\nthe base address of cabac table\n\nthe address should 128bit align"]
    #[inline(always)]
    pub fn sw_cabactbl_base(&self) -> SwCabactblBaseR {
        SwCabactblBaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - the base address of cabac table\n\nthe base address of cabac table\n\nthe address should 128bit align"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cabactbl_base(&mut self) -> SwCabactblBaseW<Swreg6CabactblProbBaseSpec> {
        SwCabactblBaseW::new(self, 4)
    }
}
#[doc = "the base address of cabac table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg6_cabactbl_prob_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg6_cabactbl_prob_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg6CabactblProbBaseSpec;
impl crate::RegisterSpec for Swreg6CabactblProbBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg6_cabactbl_prob_base::R`](R) reader structure"]
impl crate::Readable for Swreg6CabactblProbBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg6_cabactbl_prob_base::W`](W) writer structure"]
impl crate::Writable for Swreg6CabactblProbBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG6_CABACTBL_PROB_BASE to value 0"]
impl crate::Resettable for Swreg6CabactblProbBaseSpec {
    const RESET_VALUE: u32 = 0;
}
