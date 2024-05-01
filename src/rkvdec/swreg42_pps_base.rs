#[doc = "Register `SWREG42_PPS_BASE` reader"]
pub type R = crate::R<Swreg42PpsBaseSpec>;
#[doc = "Register `SWREG42_PPS_BASE` writer"]
pub type W = crate::W<Swreg42PpsBaseSpec>;
#[doc = "Field `SW_PPS_BASE` reader - the base address of pps\n\nthe base address of pps ( the address should 128bit align )\n\nit is for storing sps(sequence parameter set) and pps(picture\n\nparameter set)"]
pub type SwPpsBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_PPS_BASE` writer - the base address of pps\n\nthe base address of pps ( the address should 128bit align )\n\nit is for storing sps(sequence parameter set) and pps(picture\n\nparameter set)"]
pub type SwPpsBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - the base address of pps\n\nthe base address of pps ( the address should 128bit align )\n\nit is for storing sps(sequence parameter set) and pps(picture\n\nparameter set)"]
    #[inline(always)]
    pub fn sw_pps_base(&self) -> SwPpsBaseR {
        SwPpsBaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - the base address of pps\n\nthe base address of pps ( the address should 128bit align )\n\nit is for storing sps(sequence parameter set) and pps(picture\n\nparameter set)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pps_base(&mut self) -> SwPpsBaseW<Swreg42PpsBaseSpec> {
        SwPpsBaseW::new(self, 4)
    }
}
#[doc = "the base address of pps\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg42_pps_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg42_pps_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg42PpsBaseSpec;
impl crate::RegisterSpec for Swreg42PpsBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg42_pps_base::R`](R) reader structure"]
impl crate::Readable for Swreg42PpsBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg42_pps_base::W`](W) writer structure"]
impl crate::Writable for Swreg42PpsBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG42_PPS_BASE to value 0"]
impl crate::Resettable for Swreg42PpsBaseSpec {
    const RESET_VALUE: u32 = 0;
}
