#[doc = "Register `BTDMAS` reader"]
pub type R = crate::R<BtdmasSpec>;
#[doc = "Register `BTDMAS` writer"]
pub type W = crate::W<BtdmasSpec>;
#[doc = "Field `STARTADDR` reader - Specifies the Start Address of DMA\n\nThe address needs to be aligned by 32-bit."]
pub type StartaddrR = crate::FieldReader<u32>;
#[doc = "Field `STARTADDR` writer - Specifies the Start Address of DMA\n\nThe address needs to be aligned by 32-bit."]
pub type StartaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the Start Address of DMA\n\nThe address needs to be aligned by 32-bit."]
    #[inline(always)]
    pub fn startaddr(&self) -> StartaddrR {
        StartaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the Start Address of DMA\n\nThe address needs to be aligned by 32-bit."]
    #[inline(always)]
    #[must_use]
    pub fn startaddr(&mut self) -> StartaddrW<BtdmasSpec> {
        StartaddrW::new(self, 0)
    }
}
#[doc = "Block Transmiting DMA Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btdmas::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btdmas::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtdmasSpec;
impl crate::RegisterSpec for BtdmasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btdmas::R`](R) reader structure"]
impl crate::Readable for BtdmasSpec {}
#[doc = "`write(|w| ..)` method takes [`btdmas::W`](W) writer structure"]
impl crate::Writable for BtdmasSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTDMAS to value 0"]
impl crate::Resettable for BtdmasSpec {
    const RESET_VALUE: u32 = 0;
}
