#[doc = "Register `WIN2_LUT_ADDR` reader"]
pub type R = crate::R<Win2LutAddrSpec>;
#[doc = "Register `WIN2_LUT_ADDR` writer"]
pub type W = crate::W<Win2LutAddrSpec>;
#[doc = "Field `WIN2_LUT_ADDR` reader - the head of win2 lut address"]
pub type Win2LutAddrR = crate::FieldReader<u32>;
#[doc = "Field `WIN2_LUT_ADDR` writer - the head of win2 lut address"]
pub type Win2LutAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the head of win2 lut address"]
    #[inline(always)]
    pub fn win2_lut_addr(&self) -> Win2LutAddrR {
        Win2LutAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the head of win2 lut address"]
    #[inline(always)]
    #[must_use]
    pub fn win2_lut_addr(&mut self) -> Win2LutAddrW<Win2LutAddrSpec> {
        Win2LutAddrW::new(self, 0)
    }
}
#[doc = "Win2 lut base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_lut_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_lut_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2LutAddrSpec;
impl crate::RegisterSpec for Win2LutAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_lut_addr::R`](R) reader structure"]
impl crate::Readable for Win2LutAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`win2_lut_addr::W`](W) writer structure"]
impl crate::Writable for Win2LutAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_LUT_ADDR to value 0"]
impl crate::Resettable for Win2LutAddrSpec {
    const RESET_VALUE: u32 = 0;
}
