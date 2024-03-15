#[doc = "Register `FC_GMD_STAT` reader"]
pub type R = crate::R<FcGmdStatSpec>;
#[doc = "Field `IGMDCURRENT_GAMUT_SEQ_NUM` reader - Gamut scheduling: Current Gamut packet sequence number"]
pub type IgmdcurrentGamutSeqNumR = crate::FieldReader;
#[doc = "Field `IGMDPACKET_SEQ` reader - Gamut scheduling: Gamut packet sequence"]
pub type IgmdpacketSeqR = crate::FieldReader;
#[doc = "Field `IGMDDNEXT_FIELD` reader - Gamut scheduling: Gamut Next field"]
pub type IgmddnextFieldR = crate::BitReader;
#[doc = "Field `IGMDNO_CRNT_GBD` reader - Gamut scheduling: No current gamut data"]
pub type IgmdnoCrntGbdR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Gamut scheduling: Current Gamut packet sequence number"]
    #[inline(always)]
    pub fn igmdcurrent_gamut_seq_num(&self) -> IgmdcurrentGamutSeqNumR {
        IgmdcurrentGamutSeqNumR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - Gamut scheduling: Gamut packet sequence"]
    #[inline(always)]
    pub fn igmdpacket_seq(&self) -> IgmdpacketSeqR {
        IgmdpacketSeqR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Gamut scheduling: Gamut Next field"]
    #[inline(always)]
    pub fn igmddnext_field(&self) -> IgmddnextFieldR {
        IgmddnextFieldR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Gamut scheduling: No current gamut data"]
    #[inline(always)]
    pub fn igmdno_crnt_gbd(&self) -> IgmdnoCrntGbdR {
        IgmdnoCrntGbdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Gamut scheduling: Current Gamut packet sequence number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_gmd_stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcGmdStatSpec;
impl crate::RegisterSpec for FcGmdStatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_gmd_stat::R`](R) reader structure"]
impl crate::Readable for FcGmdStatSpec {}
#[doc = "`reset()` method sets FC_GMD_STAT to value 0"]
impl crate::Resettable for FcGmdStatSpec {
    const RESET_VALUE: u8 = 0;
}
