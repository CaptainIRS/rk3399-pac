#[doc = "Register `DPCC_BPT_DATA` reader"]
pub type R = crate::R<DpccBptDataSpec>;
#[doc = "Register `DPCC_BPT_DATA` writer"]
pub type W = crate::W<DpccBptDataSpec>;
#[doc = "Field `bpt_h_addr` reader - Bad Pixel horizontal address (pixel position)"]
pub type BptHAddrR = crate::FieldReader<u16>;
#[doc = "Field `bpt_h_addr` writer - Bad Pixel horizontal address (pixel position)"]
pub type BptHAddrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `bpt_v_addr` reader - Bad Pixel vertical address (pixel position)"]
pub type BptVAddrR = crate::FieldReader<u16>;
#[doc = "Field `bpt_v_addr` writer - Bad Pixel vertical address (pixel position)"]
pub type BptVAddrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:12 - Bad Pixel horizontal address (pixel position)"]
    #[inline(always)]
    pub fn bpt_h_addr(&self) -> BptHAddrR {
        BptHAddrR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:27 - Bad Pixel vertical address (pixel position)"]
    #[inline(always)]
    pub fn bpt_v_addr(&self) -> BptVAddrR {
        BptVAddrR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Bad Pixel horizontal address (pixel position)"]
    #[inline(always)]
    #[must_use]
    pub fn bpt_h_addr(&mut self) -> BptHAddrW<DpccBptDataSpec> {
        BptHAddrW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Bad Pixel vertical address (pixel position)"]
    #[inline(always)]
    #[must_use]
    pub fn bpt_v_addr(&mut self) -> BptVAddrW<DpccBptDataSpec> {
        BptVAddrW::new(self, 16)
    }
}
#[doc = "TABLE DATA register for read and write access of table RAM\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\n\n\nversion of SIG \n\n\n\n-> rwhh \n\n\n\nThe programmed table value is immediately written into the RAM. The RAM address is \n\n\n\ngenerated per auto-increment. The parameter RAMs for Lens Shade Correction and Bad \n\n\n\nPixel Correction can only be programmed, if the RGB Bayer path is switched on via ISP_CTRL \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_bpt_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_bpt_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccBptDataSpec;
impl crate::RegisterSpec for DpccBptDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_bpt_data::R`](R) reader structure"]
impl crate::Readable for DpccBptDataSpec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_bpt_data::W`](W) writer structure"]
impl crate::Writable for DpccBptDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_BPT_DATA to value 0"]
impl crate::Resettable for DpccBptDataSpec {
    const RESET_VALUE: u32 = 0;
}
