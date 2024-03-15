#[doc = "Register `SDMMC_DBADDR` reader"]
pub type R = crate::R<SdmmcDbaddrSpec>;
#[doc = "Register `SDMMC_DBADDR` writer"]
pub type W = crate::W<SdmmcDbaddrSpec>;
#[doc = "Field `SDL` reader - Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits \\[0/1/2:0\\]
for 16/32/64-bit bus-width) are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits are read-only."]
pub type SdlR = crate::FieldReader<u32>;
#[doc = "Field `SDL` writer - Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits \\[0/1/2:0\\]
for 16/32/64-bit bus-width) are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits are read-only."]
pub type SdlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits \\[0/1/2:0\\]
for 16/32/64-bit bus-width) are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits are read-only."]
    #[inline(always)]
    pub fn sdl(&self) -> SdlR {
        SdlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits \\[0/1/2:0\\]
for 16/32/64-bit bus-width) are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn sdl(&mut self) -> SdlW<SdmmcDbaddrSpec> {
        SdlW::new(self, 0)
    }
}
#[doc = "Descriptor list base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_dbaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_dbaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcDbaddrSpec;
impl crate::RegisterSpec for SdmmcDbaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_dbaddr::R`](R) reader structure"]
impl crate::Readable for SdmmcDbaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_dbaddr::W`](W) writer structure"]
impl crate::Writable for SdmmcDbaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_DBADDR to value 0"]
impl crate::Resettable for SdmmcDbaddrSpec {
    const RESET_VALUE: u32 = 0;
}
