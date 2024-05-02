#[doc = "Register `MI_DMA_CTRL` reader"]
pub type R = crate::R<MiDmaCtrlSpec>;
#[doc = "Register `MI_DMA_CTRL` writer"]
pub type W = crate::W<MiDmaCtrlSpec>;
#[doc = "Selects RGB Bayer data of read DMA picture 00: no\n\nDMA RGB Bayer data\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaRgbFormat {
    #[doc = "1: 8 bit RGB Bayer data"]
    B01 = 1,
    #[doc = "2: 16 bit RGB Bayer data (12 bit used) bytes are organized MSB first and 4 lower bits of LSB remain unused: byte_even -> bayer\\[11:4\\], byte_odd\\[7:4\\]
-> bayer\\[3:0\\]
11: reserved."]
    B10 = 2,
}
impl From<DmaRgbFormat> for u8 {
    #[inline(always)]
    fn from(variant: DmaRgbFormat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmaRgbFormat {
    type Ux = u8;
}
#[doc = "Field `dma_rgb_format` reader - Selects RGB Bayer data of read DMA picture 00: no\n\nDMA RGB Bayer data"]
pub type DmaRgbFormatR = crate::FieldReader<DmaRgbFormat>;
impl DmaRgbFormatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DmaRgbFormat> {
        match self.bits {
            1 => Some(DmaRgbFormat::B01),
            2 => Some(DmaRgbFormat::B10),
            _ => None,
        }
    }
    #[doc = "8 bit RGB Bayer data"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DmaRgbFormat::B01
    }
    #[doc = "16 bit RGB Bayer data (12 bit used) bytes are organized MSB first and 4 lower bits of LSB remain unused: byte_even -> bayer\\[11:4\\], byte_odd\\[7:4\\]
-> bayer\\[3:0\\]
11: reserved."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == DmaRgbFormat::B10
    }
}
#[doc = "Field `dma_rgb_format` writer - Selects RGB Bayer data of read DMA picture 00: no\n\nDMA RGB Bayer data"]
pub type DmaRgbFormatW<'a, REG> = crate::FieldWriter<'a, REG, 2, DmaRgbFormat>;
impl<'a, REG> DmaRgbFormatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bit RGB Bayer data"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRgbFormat::B01)
    }
    #[doc = "16 bit RGB Bayer data (12 bit used) bytes are organized MSB first and 4 lower bits of LSB remain unused: byte_even -> bayer\\[11:4\\], byte_odd\\[7:4\\]
-> bayer\\[3:0\\]
11: reserved."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRgbFormat::B10)
    }
}
impl R {
    #[doc = "Bits 12:13 - Selects RGB Bayer data of read DMA picture 00: no\n\nDMA RGB Bayer data"]
    #[inline(always)]
    pub fn dma_rgb_format(&self) -> DmaRgbFormatR {
        DmaRgbFormatR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13 - Selects RGB Bayer data of read DMA picture 00: no\n\nDMA RGB Bayer data"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rgb_format(&mut self) -> DmaRgbFormatW<MiDmaCtrlSpec> {
        DmaRgbFormatW::new(self, 12)
    }
}
#[doc = "DMA control register\n\nNote: The dma_ready \n\ninterrupt is raised as usual, but the dma_frame_end \n\ninterrupt will not be generated until v_end has been \n\nenabled again. \n\n\n\n9 dma_continuous_en Enables continuous mode. If set the same frame is \n\nread back over and over. A start pulse on dma_start is \n\nneeded only for the first time. To stop continuous mode \n\nreset this bit (takes effect after the next frame end) or \n\nexecute a soft reset. This bit is intended to be used in \n\nconjunction with the Superimpose feature. \n\n\n\n8 dma_byte_swap Enables change of DMA byte order of the 32 bit \n\n\n\ninput word at read port \n\n1: byte order is mirrored but the bit order within one \n\n\n\nbyte doesn‟t change \n\n\n\n0: no byte mirroring \n\n\n\n7:6 dma_inout_format Selects input/output format of \n\n\n\nDMA picture. 11: YCbCr 4:4:4 \n\n\n\n10: YCbCr 4:2:2 \n\n\n\n01: YCbCr 4:2:0 \n\n\n\n00: YCbCr 4:0:0 \n\n\n\n5:4 dma_read_format Defines how YCbCr picture data is read from \n\n\n\nmemory. 00: planar \n\n01: semi planar, for YCbCr 4:2:x \n\n10: interleaved (combined), for YCbCr 4:2:2 and RGB \n\n\n\nonly 11: reserved \n\n\n\n3:2 dma_burst_len_chrom Burst length for Cb or Cr data affecting DMA \n\n\n\nread port. 00: 4-beat bursts \n\n\n\n01: 8-beat bursts \n\n\n\n10: 16-beat bursts \n\n11: reserved \n\nIgnored if 8- or 16-beat bursts are not supported. \n\n\n\nDMA control register Reset value: 0000'0000H \n\n \n\n\n\nAddress: ISP_MI_BASE + 0120H Mode : rw \n\n\n\nBit\n\n\n\ns \n\n\n\nName Description \n\n\n\n1:0 dma_burst_len_lum Burst length for Y data affecting DMA read port. \n\n\n\n00: 4-beat bursts \n\n\n\n01: 8-beat bursts \n\n10: 16-beat bursts \n\n\n\n11: reserved \n\n\n\nIgnored if 8- or 16-beat bursts are not supported. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiDmaCtrlSpec;
impl crate::RegisterSpec for MiDmaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_dma_ctrl::R`](R) reader structure"]
impl crate::Readable for MiDmaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_dma_ctrl::W`](W) writer structure"]
impl crate::Writable for MiDmaCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_DMA_CTRL to value 0"]
impl crate::Resettable for MiDmaCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
