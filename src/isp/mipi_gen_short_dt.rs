#[doc = "Register `MIPI_GEN_SHORT_DT` reader"]
pub type R = crate::R<MipiGenShortDtSpec>;
#[doc = "Field `GEN_SHORT_DT_0x8` reader - 1: generic short packet of data type 0x8\n\nreceived 0: data type 0x8 not received"]
pub type GenShortDt0x8R = crate::BitReader;
#[doc = "Field `GEN_SHORT_DT_0x9` reader - 1: generic short packet of data type 0x9\n\nreceived 0: data type 0x9 not received"]
pub type GenShortDt0x9R = crate::BitReader;
#[doc = "Field `GEN_SHORT_DT_0xA` reader - 1: generic short packet of data type 0xA\n\nreceived 0: data type 0xA not received"]
pub type GenShortDt0xAR = crate::BitReader;
#[doc = "Field `GEN_SHORT_DT_0xB` reader - 1: generic short packet of data type 0xB\n\nreceived 0: data type 0xB not received"]
pub type GenShortDt0xBR = crate::BitReader;
#[doc = "Field `GEN_SHORT_DT_0xC` reader - 1: generic short packet of data type 0xC\n\nreceived 0: data type 0xC not received"]
pub type GenShortDt0xCR = crate::BitReader;
#[doc = "Field `GEN_SHORT_DT_0xD` reader - 1: generic short packet of data type 0xD\n\nreceived 0: data type 0xD not received"]
pub type GenShortDt0xDR = crate::BitReader;
#[doc = "Field `GEN_SHORT_DT_0xE` reader - 1: generic short packet of data type 0xE received 0:\n\ndata type 0xE not received"]
pub type GenShortDt0xER = crate::BitReader;
#[doc = "Field `GEN_SHORT_DT_0xF` reader - 1: generic short packet of data type 0xF received 0:\n\ndata type 0xF not received"]
pub type GenShortDt0xFR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: generic short packet of data type 0x8\n\nreceived 0: data type 0x8 not received"]
    #[inline(always)]
    pub fn gen_short_dt_0x8(&self) -> GenShortDt0x8R {
        GenShortDt0x8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: generic short packet of data type 0x9\n\nreceived 0: data type 0x9 not received"]
    #[inline(always)]
    pub fn gen_short_dt_0x9(&self) -> GenShortDt0x9R {
        GenShortDt0x9R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: generic short packet of data type 0xA\n\nreceived 0: data type 0xA not received"]
    #[inline(always)]
    pub fn gen_short_dt_0x_a(&self) -> GenShortDt0xAR {
        GenShortDt0xAR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: generic short packet of data type 0xB\n\nreceived 0: data type 0xB not received"]
    #[inline(always)]
    pub fn gen_short_dt_0x_b(&self) -> GenShortDt0xBR {
        GenShortDt0xBR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: generic short packet of data type 0xC\n\nreceived 0: data type 0xC not received"]
    #[inline(always)]
    pub fn gen_short_dt_0x_c(&self) -> GenShortDt0xCR {
        GenShortDt0xCR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: generic short packet of data type 0xD\n\nreceived 0: data type 0xD not received"]
    #[inline(always)]
    pub fn gen_short_dt_0x_d(&self) -> GenShortDt0xDR {
        GenShortDt0xDR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: generic short packet of data type 0xE received 0:\n\ndata type 0xE not received"]
    #[inline(always)]
    pub fn gen_short_dt_0x_e(&self) -> GenShortDt0xER {
        GenShortDt0xER::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: generic short packet of data type 0xF received 0:\n\ndata type 0xF not received"]
    #[inline(always)]
    pub fn gen_short_dt_0x_f(&self) -> GenShortDt0xFR {
        GenShortDt0xFR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "data type flags for received generic short packets\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_gen_short_dt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiGenShortDtSpec;
impl crate::RegisterSpec for MipiGenShortDtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_gen_short_dt::R`](R) reader structure"]
impl crate::Readable for MipiGenShortDtSpec {}
#[doc = "`reset()` method sets MIPI_GEN_SHORT_DT to value 0"]
impl crate::Resettable for MipiGenShortDtSpec {
    const RESET_VALUE: u32 = 0;
}
