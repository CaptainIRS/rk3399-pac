#[doc = "Register `GEN_HDR` reader"]
pub type R = crate::R<GenHdrSpec>;
#[doc = "Register `GEN_HDR` writer"]
pub type W = crate::W<GenHdrSpec>;
#[doc = "Field `GEN_DT` reader - gen_dt\n\nThis field configures the packet data type of the header packet."]
pub type GenDtR = crate::FieldReader;
#[doc = "Field `GEN_DT` writer - gen_dt\n\nThis field configures the packet data type of the header packet."]
pub type GenDtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `GEN_VC` reader - gen_vc\n\nThis field configures the virtual channel id of the header packet."]
pub type GenVcR = crate::FieldReader;
#[doc = "Field `GEN_VC` writer - gen_vc\n\nThis field configures the virtual channel id of the header packet."]
pub type GenVcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GEN_WC_LSBYTE` reader - gen_wc_lsbyte\n\nThis field configures the least significant byte of the header packet's\n\nWord count for long packets or data 0 for short packets"]
pub type GenWcLsbyteR = crate::FieldReader;
#[doc = "Field `GEN_WC_LSBYTE` writer - gen_wc_lsbyte\n\nThis field configures the least significant byte of the header packet's\n\nWord count for long packets or data 0 for short packets"]
pub type GenWcLsbyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GEN_WC_MSBYTE` reader - gen_wc_msbyte\n\ngen_wc_msbyte"]
pub type GenWcMsbyteR = crate::FieldReader;
#[doc = "Field `GEN_WC_MSBYTE` writer - gen_wc_msbyte\n\ngen_wc_msbyte"]
pub type GenWcMsbyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - gen_dt\n\nThis field configures the packet data type of the header packet."]
    #[inline(always)]
    pub fn gen_dt(&self) -> GenDtR {
        GenDtR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - gen_vc\n\nThis field configures the virtual channel id of the header packet."]
    #[inline(always)]
    pub fn gen_vc(&self) -> GenVcR {
        GenVcR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - gen_wc_lsbyte\n\nThis field configures the least significant byte of the header packet's\n\nWord count for long packets or data 0 for short packets"]
    #[inline(always)]
    pub fn gen_wc_lsbyte(&self) -> GenWcLsbyteR {
        GenWcLsbyteR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - gen_wc_msbyte\n\ngen_wc_msbyte"]
    #[inline(always)]
    pub fn gen_wc_msbyte(&self) -> GenWcMsbyteR {
        GenWcMsbyteR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - gen_dt\n\nThis field configures the packet data type of the header packet."]
    #[inline(always)]
    #[must_use]
    pub fn gen_dt(&mut self) -> GenDtW<GenHdrSpec> {
        GenDtW::new(self, 0)
    }
    #[doc = "Bits 6:7 - gen_vc\n\nThis field configures the virtual channel id of the header packet."]
    #[inline(always)]
    #[must_use]
    pub fn gen_vc(&mut self) -> GenVcW<GenHdrSpec> {
        GenVcW::new(self, 6)
    }
    #[doc = "Bits 8:15 - gen_wc_lsbyte\n\nThis field configures the least significant byte of the header packet's\n\nWord count for long packets or data 0 for short packets"]
    #[inline(always)]
    #[must_use]
    pub fn gen_wc_lsbyte(&mut self) -> GenWcLsbyteW<GenHdrSpec> {
        GenWcLsbyteW::new(self, 8)
    }
    #[doc = "Bits 16:23 - gen_wc_msbyte\n\ngen_wc_msbyte"]
    #[inline(always)]
    #[must_use]
    pub fn gen_wc_msbyte(&mut self) -> GenWcMsbyteW<GenHdrSpec> {
        GenWcMsbyteW::new(self, 16)
    }
}
#[doc = "Generic Packet Header Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_hdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_hdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GenHdrSpec;
impl crate::RegisterSpec for GenHdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_hdr::R`](R) reader structure"]
impl crate::Readable for GenHdrSpec {}
#[doc = "`write(|w| ..)` method takes [`gen_hdr::W`](W) writer structure"]
impl crate::Writable for GenHdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN_HDR to value 0"]
impl crate::Resettable for GenHdrSpec {
    const RESET_VALUE: u32 = 0;
}
