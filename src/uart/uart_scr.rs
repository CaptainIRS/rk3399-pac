#[doc = "Register `UART_SCR` reader"]
pub type R = crate::R<UartScrSpec>;
#[doc = "Register `UART_SCR` writer"]
pub type W = crate::W<UartScrSpec>;
#[doc = "Field `TEMP_STORE_SPACE` reader - This register is for programmers to use as a temporary storage\n\nspace."]
pub type TempStoreSpaceR = crate::FieldReader;
#[doc = "Field `TEMP_STORE_SPACE` writer - This register is for programmers to use as a temporary storage\n\nspace."]
pub type TempStoreSpaceW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register is for programmers to use as a temporary storage\n\nspace."]
    #[inline(always)]
    pub fn temp_store_space(&self) -> TempStoreSpaceR {
        TempStoreSpaceR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is for programmers to use as a temporary storage\n\nspace."]
    #[inline(always)]
    #[must_use]
    pub fn temp_store_space(&mut self) -> TempStoreSpaceW<UartScrSpec> {
        TempStoreSpaceW::new(self, 0)
    }
}
#[doc = "Scratchpad Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartScrSpec;
impl crate::RegisterSpec for UartScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_scr::R`](R) reader structure"]
impl crate::Readable for UartScrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_scr::W`](W) writer structure"]
impl crate::Writable for UartScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_SCR to value 0"]
impl crate::Resettable for UartScrSpec {
    const RESET_VALUE: u32 = 0;
}
