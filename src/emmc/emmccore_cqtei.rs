#[doc = "Register `EMMCCORE_CQTEI` reader"]
pub type R = crate::R<EmmccoreCqteiSpec>;
#[doc = "Field `RMECI` reader - Response Mode Error Command Index This field indicates the index of the command which was executed on the command line when an error occurred. The field is updated if a command transaction is in progress when an error is detected by CQE, or indicated by eMMC controller."]
pub type RmeciR = crate::FieldReader;
#[doc = "Field `RMETID` reader - Response Mode Error Task ID This field indicates the ID of the task which was executed on the command line when an error occurred. The field is updated if a command transaction is in progress when an error is detected by CQE, or indicated by eMMC controller."]
pub type RmetidR = crate::FieldReader;
#[doc = "Field `RMEFV` reader - Response Mode Error Fields Valid This bit is updated when an error is detected by CQE, or indicated by eMMC controller. If a command transaction is in progress when the error is detected/indicated, the bit is set to 1. If a no command transaction is in progress when the error is detected/indicated, the bit is cleared to 0."]
pub type RmefvR = crate::BitReader;
#[doc = "Field `DTECI` reader - Data Transfer Error Command Index This field indicates the index of the command which was executed on the data lines when an error occurred. The index shall be set to EXECUTE_READ_TASK (CMD46) or EXECUTE_WRITE_TASK (CMD47) according to the data direction. The field is updated if a data transfer is in progress when an error is detected by CQE, or indicated by eMMC controller."]
pub type DteciR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Response Mode Error Command Index This field indicates the index of the command which was executed on the command line when an error occurred. The field is updated if a command transaction is in progress when an error is detected by CQE, or indicated by eMMC controller."]
    #[inline(always)]
    pub fn rmeci(&self) -> RmeciR {
        RmeciR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Response Mode Error Task ID This field indicates the ID of the task which was executed on the command line when an error occurred. The field is updated if a command transaction is in progress when an error is detected by CQE, or indicated by eMMC controller."]
    #[inline(always)]
    pub fn rmetid(&self) -> RmetidR {
        RmetidR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Response Mode Error Fields Valid This bit is updated when an error is detected by CQE, or indicated by eMMC controller. If a command transaction is in progress when the error is detected/indicated, the bit is set to 1. If a no command transaction is in progress when the error is detected/indicated, the bit is cleared to 0."]
    #[inline(always)]
    pub fn rmefv(&self) -> RmefvR {
        RmefvR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Data Transfer Error Command Index This field indicates the index of the command which was executed on the data lines when an error occurred. The index shall be set to EXECUTE_READ_TASK (CMD46) or EXECUTE_WRITE_TASK (CMD47) according to the data direction. The field is updated if a data transfer is in progress when an error is detected by CQE, or indicated by eMMC controller."]
    #[inline(always)]
    pub fn dteci(&self) -> DteciR {
        DteciR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "Command queueing task error information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqtei::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqteiSpec;
impl crate::RegisterSpec for EmmccoreCqteiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqtei::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqteiSpec {}
#[doc = "`reset()` method sets EMMCCORE_CQTEI to value 0"]
impl crate::Resettable for EmmccoreCqteiSpec {
    const RESET_VALUE: u32 = 0;
}
