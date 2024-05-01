#[doc = "Register `WIN3_LUT_ADDR` reader"]
pub type R = crate::R<Win3LutAddrSpec>;
#[doc = "Register `WIN3_LUT_ADDR` writer"]
pub type W = crate::W<Win3LutAddrSpec>;
#[doc = "Field `WIN3_LUT_ADDR` reader - the head of win3 lut address"]
pub type Win3LutAddrR = crate::FieldReader<u32>;
#[doc = "Field `WIN3_LUT_ADDR` writer - the head of win3 lut address"]
pub type Win3LutAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the head of win3 lut address"]
    #[inline(always)]
    pub fn win3_lut_addr(&self) -> Win3LutAddrR {
        Win3LutAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the head of win3 lut address"]
    #[inline(always)]
    #[must_use]
    pub fn win3_lut_addr(&mut self) -> Win3LutAddrW<Win3LutAddrSpec> {
        Win3LutAddrW::new(self, 0)
    }
}
#[doc = "Win3 lut base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_lut_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_lut_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3LutAddrSpec;
impl crate::RegisterSpec for Win3LutAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_lut_addr::R`](R) reader structure"]
impl crate::Readable for Win3LutAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`win3_lut_addr::W`](W) writer structure"]
impl crate::Writable for Win3LutAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_LUT_ADDR to value 0"]
impl crate::Resettable for Win3LutAddrSpec {
    const RESET_VALUE: u32 = 0;
}
