<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: gw/gw.proto

namespace Chirpstack\Gateway;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>gw.DownlinkTxAckItem</code>
 */
class DownlinkTxAckItem extends \Google\Protobuf\Internal\Message
{
    /**
     * The Ack status of this item.
     *
     * Generated from protobuf field <code>.gw.TxAckStatus status = 1;</code>
     */
    protected $status = 0;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type int $status
     *           The Ack status of this item.
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Chirpstack\Gateway\Gw::initOnce();
        parent::__construct($data);
    }

    /**
     * The Ack status of this item.
     *
     * Generated from protobuf field <code>.gw.TxAckStatus status = 1;</code>
     * @return int
     */
    public function getStatus()
    {
        return $this->status;
    }

    /**
     * The Ack status of this item.
     *
     * Generated from protobuf field <code>.gw.TxAckStatus status = 1;</code>
     * @param int $var
     * @return $this
     */
    public function setStatus($var)
    {
        GPBUtil::checkEnum($var, \Chirpstack\Gateway\TxAckStatus::class);
        $this->status = $var;

        return $this;
    }

}
