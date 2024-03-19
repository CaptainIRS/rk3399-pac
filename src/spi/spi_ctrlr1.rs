#[doc = "Register `SPI_CTRLR1` reader"]
pub type R = crate::R<SpiCtrlr1Spec>;
#[doc = "Register `SPI_CTRLR1` writer"]
pub type W = crate::W<SpiCtrlr1Spec>;
#[doc = "Field `NDM` reader - Number of Data Frames\n\nWhen Transfer Mode is receive only, this register field sets the\n\nnumber of data frames to be continuously received by the SPI.\n\nThe SPI continues to receive serial data until the number of data\n\nframes received is equal to this register value plus 1, which\n\nenables you to receive up to 64 KB of data in a continuous\n\ntransfer."]
pub type NdmR = crate::FieldReader<u16>;
#[doc = "Field `NDM` writer - Number of Data Frames\n\nWhen Transfer Mode is receive only, this register field sets the\n\nnumber of data frames to be continuously received by the SPI.\n\nThe SPI continues to receive serial data until the number of data\n\nframes received is equal to this register value plus 1, which\n\nenables you to receive up to 64 KB of data in a continuous\n\ntransfer."]
pub type NdmW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of Data Frames\n\nWhen Transfer Mode is receive only, this register field sets the\n\nnumber of data frames to be continuously received by the SPI.\n\nThe SPI continues to receive serial data until the number of data\n\nframes received is equal to this register value plus 1, which\n\nenables you to receive up to 64 KB of data in a continuous\n\ntransfer."]
    #[inline(always)]
    pub fn ndm(&self) -> NdmR {
        NdmR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of Data Frames\n\nWhen Transfer Mode is receive only, this register field sets the\n\nnumber of data frames to be continuously received by the SPI.\n\nThe SPI continues to receive serial data until the number of data\n\nframes received is equal to this register value plus 1, which\n\nenables you to receive up to 64 KB of data in a continuous\n\ntransfer."]
    #[inline(always)]
    #[must_use]
    pub fn ndm(&mut self) -> NdmW<SpiCtrlr1Spec> {
        NdmW::new(self, 0)
    }
}
#[doc = "Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ctrlr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ctrlr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiCtrlr1Spec;
impl crate::RegisterSpec for SpiCtrlr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ctrlr1::R`](R) reader structure"]
impl crate::Readable for SpiCtrlr1Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_ctrlr1::W`](W) writer structure"]
impl crate::Writable for SpiCtrlr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_CTRLR1 to value 0"]
impl crate::Resettable for SpiCtrlr1Spec {
    const RESET_VALUE: u32 = 0;
}
