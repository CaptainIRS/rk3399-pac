#[doc = "Register `SWREG43_RPS_BASE` reader"]
pub type R = crate::R<Swreg43RpsBaseSpec>;
#[doc = "Register `SWREG43_RPS_BASE` writer"]
pub type W = crate::W<Swreg43RpsBaseSpec>;
#[doc = "Field `SW_RPS_BASE` reader - rps base address\n\nrps(reference picture set) base address (the address should 128bit\n\nalign)"]
pub type SwRpsBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_RPS_BASE` writer - rps base address\n\nrps(reference picture set) base address (the address should 128bit\n\nalign)"]
pub type SwRpsBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - rps base address\n\nrps(reference picture set) base address (the address should 128bit\n\nalign)"]
    #[inline(always)]
    pub fn sw_rps_base(&self) -> SwRpsBaseR {
        SwRpsBaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - rps base address\n\nrps(reference picture set) base address (the address should 128bit\n\nalign)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rps_base(&mut self) -> SwRpsBaseW<Swreg43RpsBaseSpec> {
        SwRpsBaseW::new(self, 4)
    }
}
#[doc = "the base address of rps\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg43_rps_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg43_rps_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg43RpsBaseSpec;
impl crate::RegisterSpec for Swreg43RpsBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg43_rps_base::R`](R) reader structure"]
impl crate::Readable for Swreg43RpsBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg43_rps_base::W`](W) writer structure"]
impl crate::Writable for Swreg43RpsBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG43_RPS_BASE to value 0"]
impl crate::Resettable for Swreg43RpsBaseSpec {
    const RESET_VALUE: u32 = 0;
}
