#[doc = "Register `HIST_BIN%s` reader"]
pub type R = crate::R<HistBinSpec>;
#[doc = "Field `hist_bin_n` reader - measured bin count as 16-bit unsigned integer\n\nvalue plus 4 bit fractional part"]
pub type HistBinNR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - measured bin count as 16-bit unsigned integer\n\nvalue plus 4 bit fractional part"]
    #[inline(always)]
    pub fn hist_bin_n(&self) -> HistBinNR {
        HistBinNR::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "histogram measurement result bin n (n=0..15)\n\nNote: MKOE tbc: Orignial register mode was rh which is no longer supported with new \n\nversion of SIG -> r \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_bin::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistBinSpec;
impl crate::RegisterSpec for HistBinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_bin::R`](R) reader structure"]
impl crate::Readable for HistBinSpec {}
#[doc = "`reset()` method sets HIST_BIN%s to value 0"]
impl crate::Resettable for HistBinSpec {
    const RESET_VALUE: u32 = 0;
}
