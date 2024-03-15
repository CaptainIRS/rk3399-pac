#[doc = "Register `GRF_FAST_BOOT_ADDR` reader"]
pub type R = crate::R<GrfFastBootAddrSpec>;
#[doc = "Register `GRF_FAST_BOOT_ADDR` writer"]
pub type W = crate::W<GrfFastBootAddrSpec>;
#[doc = "Field `FAST_BOOT_ADDR` reader - fast boot address"]
pub type FastBootAddrR = crate::FieldReader<u32>;
#[doc = "Field `FAST_BOOT_ADDR` writer - fast boot address"]
pub type FastBootAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fast boot address"]
    #[inline(always)]
    pub fn fast_boot_addr(&self) -> FastBootAddrR {
        FastBootAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fast boot address"]
    #[inline(always)]
    #[must_use]
    pub fn fast_boot_addr(&mut self) -> FastBootAddrW<GrfFastBootAddrSpec> {
        FastBootAddrW::new(self, 0)
    }
}
#[doc = "faster boot address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_fast_boot_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_fast_boot_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfFastBootAddrSpec;
impl crate::RegisterSpec for GrfFastBootAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_fast_boot_addr::R`](R) reader structure"]
impl crate::Readable for GrfFastBootAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_fast_boot_addr::W`](W) writer structure"]
impl crate::Writable for GrfFastBootAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_FAST_BOOT_ADDR to value 0"]
impl crate::Resettable for GrfFastBootAddrSpec {
    const RESET_VALUE: u32 = 0;
}
